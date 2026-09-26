# MegaStore — Sistema de Recomendação Baseado em Grafos

Projeto desenvolvido para a disciplina **Data Structure Strategy and Implementation** (UniFECAF), com o objetivo de substituir recomendações genéricas (mais vendidos / mesma categoria) por recomendações relevantes, geradas a partir da modelagem de clientes e produtos como um **grafo**.

## Índice

- [Objetivo e Funcionamento](#objetivo-e-funcionamento)
- [Tecnologias e Estruturas de Dados](#tecnologias-e-estruturas-de-dados)
- [Arquitetura](#arquitetura)
- [Instruções de Compilação, Execução e Testes](#instruções-de-compilação-execução-e-testes)
- [Exemplos de Uso](#exemplos-de-uso)
- [BFS vs. Dijkstra: uma comparação](#bfs-vs-dijkstra-uma-comparação)
- [O Índice Invertido: otimizando a busca](#o-índice-invertido-otimizando-a-busca)
- [Resultados de Desempenho](#resultados-de-desempenho)
- [Testes](#testes)

## Objetivo e Funcionamento

O MegaStore é um e-commerce fictício com um catálogo muito grande de produtos. O problema proposto pelo edital é: **como gerar recomendações relevantes para cada cliente, sem depender apenas de "mais vendidos" ou "mesma categoria"?**

A solução implementada modela o problema como um **grafo bipartido Cliente–Produto**:

- **Vértices**: clientes e produtos (dois tipos distintos de vértice)
- **Arestas**: uma compra, ligando um cliente a um produto, com peso associado (ex: quantidade ou nota de avaliação)
- **Similaridade produto-produto** não é uma aresta direta — ela **emerge** ao percorrer o grafo: dois produtos são "parecidos" se clientes parecidos os compraram.

O sistema oferece recomendação em **dois sentidos**:
- **A partir de um cliente**: "o que mais este cliente pode gostar?", baseado em clientes parecidos (que compraram produtos em comum).
- **A partir de um produto**: "quem comprou este produto, o que mais comprou?", útil para telas de "produtos relacionados".

## Tecnologias e Estruturas de Dados

- **Linguagem**: Rust (apenas biblioteca padrão, sem dependências externas)

| Estrutura | Uso | Justificativa |
|---|---|---|
| `HashMap<u32, Produto>` / `HashMap<u32, Cliente>` | Cadastro e consulta de produtos/clientes por id | Consulta O(1), essencial com catálogos grandes |
| `HashMap<u32, Vec<(u32, f64)>>` (`arestas`) | Lista de adjacência (cliente → produtos comprados) | Preferida a matriz de adjacência: com milhões de produtos, uma matriz N×N seria majoritariamente vazia (grafo esparso), desperdiçando memória |
| `HashMap<u32, Vec<u32>>` (`indice_produto_clientes`) | Índice invertido (produto → clientes que compraram) | Evita varredura O(N) por todos os clientes ao buscar quem comprou um produto; troca espaço por tempo (ver seção dedicada) |
| `VecDeque` | Fila do algoritmo de busca (BFS) | Permite inserir no fim e remover do início em O(1), essencial para processar os vértices em camadas (ordem correta de uma busca em largura) |
| `HashSet` | Controle de visitados + prevenção de recomendação duplicada | Evita reprocessar o mesmo vértice (loop infinito) e garante que um produto já comprado pelo cliente, ou já recomendado, não apareça de novo |
| `BinaryHeap` | Fila de prioridade do Dijkstra | Permite sempre processar o vértice de menor custo acumulado primeiro (implementado como min-heap via `Ord` invertido) |

### Por que dois algoritmos: BFS e Dijkstra?

A **Busca em Largura (BFS)** é a base do sistema: explora a vizinhança do cliente ou produto em camadas discretas (1º grau: produtos comprados; 2º grau: clientes parecidos; 3º grau: produtos desses clientes), sem diferenciar a força de cada conexão — todo "cliente parecido" conta igual.

O **Dijkstra** foi implementado como uma segunda opção, para comparação: cada aresta recebe um custo (`1.0 / peso_da_compra`), de forma que conexões mais fortes (mais unidades compradas, notas mais altas) tenham custo menor e sejam priorizadas. Isso permite recomendações mais sensíveis à intensidade da preferência do cliente. A comparação entre os dois está detalhada [na seção correspondente](#bfs-vs-dijkstra-uma-comparação).

## Arquitetura

```
megastore/
├── Cargo.toml
├── src/
│   ├── lib.rs              # expõe os módulos como biblioteca (usado pelos testes de integração)
│   ├── main.rs              # ponto de entrada, demonstrações de uso
│   ├── modelo/
│   │   ├── mod.rs
│   │   ├── produto.rs        # struct Produto
│   │   └── cliente.rs        # struct Cliente
│   ├── grafo/
│   │   ├── mod.rs             # struct Grafo (produtos, clientes, arestas, índice invertido), cadastro e consulta, testes unitários
│   │   ├── busca.rs            # BFS: recomendar (a partir de cliente) e recomendar_por_produto (a partir de produto), testes unitários
│   │   └── dijkstra.rs          # Dijkstra: recomendar_dijkstra
│   └── benchmark/
│       └── mod.rs               # geração de dados fictícios e medição de desempenho
└── tests/
    └── recomendacao.rs          # testes de integração (caminho feliz + caso negativo)
```

**Fluxo de dados**: `main.rs` monta um `Grafo`, populando-o com `Produto`s e `Cliente`s e registrando compras como arestas (o que também atualiza o índice invertido). As funções em `grafo::busca` e `grafo::dijkstra` percorrem esse grafo, a partir de um cliente ou de um produto, e devolvem uma lista de recomendações ordenada por relevância.

## Instruções de Compilação, Execução e Testes

Pré-requisito: [Rust e Cargo](https://www.rust-lang.org/tools/install) instalados.

```bash
# Compilar e rodar o programa principal (inclui demonstrações + benchmark)
cargo run

# Rodar em modo otimizado (recomendado para volumes grandes de dados)
cargo run --release

# Rodar todos os testes (unitários + integração)
cargo test
```

## Exemplos de Uso

### Cadastro, consulta e recomendação a partir de um cliente

```rust
let mut g = Grafo::new();

g.adicionar_produto(Produto::new(1, "Fone Bluetooth", "Eletrônicos", 129.90));
g.adicionar_produto(Produto::new(2, "Carregador", "Eletrônicos", 39.90));
g.adicionar_produto(Produto::new(3, "Mouse", "Eletrônicos", 59.90));

g.adicionar_cliente(Cliente::new(1, "Ana Souza", "Goiânia"));
g.adicionar_cliente(Cliente::new(2, "Bruno Lima", "Anápolis"));

g.adicionar_compra(1, 1, 3.0); // Ana comprou Fone
g.adicionar_compra(1, 2, 2.0); // Ana comprou Carregador
g.adicionar_compra(2, 1, 4.0); // Bruno comprou Fone
g.adicionar_compra(2, 3, 1.0); // Bruno comprou Mouse

// Consulta
if let Some(produto) = g.consultar_produto(1) {
    produto.exibir();
}

// Recomendação a partir do cliente
let recomendacoes = grafo::busca::recomendar(&g, 1, 3);
// -> [(3, 1)]  → recomenda o Mouse (produto 3) para Ana, com score 1
```

Ana e Bruno compraram um produto em comum (Fone Bluetooth), então são considerados "clientes parecidos". Como Bruno também tem o Mouse e Ana não, o Mouse é recomendado para Ana — e o Carregador, que Ana já possui, corretamente **não** é recomendado de volta.

### Recomendação a partir de um produto

```rust
// "Quem comprou o Fone Bluetooth também comprou:"
let relacionados = grafo::busca::recomendar_por_produto(&g, 1, 2);
// -> [(2, 1), (3, 1)]  → Carregador e Mouse, cada um comprado por um cliente que também tem o Fone
```

## BFS vs. Dijkstra: uma comparação

Para ilustrar a diferença entre os dois algoritmos, considere um cenário com 3 clientes: Ana compra o Fone Bluetooth; Bruno também compra o Fone, mas com uma conexão **fraca** (peso 1.0), e compra o Mouse; Carla também compra o Fone, mas com uma conexão **forte** (peso 10.0), e compra o Teclado.

| Algoritmo | Resultado | Interpretação |
|---|---|---|
| BFS | Mouse e Teclado empatados, score 1 cada | O BFS conta apenas quantos clientes parecidos levaram a cada produto, sem diferenciar a força da conexão. A ordem entre empates varia entre execuções (ver nota abaixo) |
| Dijkstra | Teclado (custo 1.3) antes de Mouse (custo 2.2) | O Dijkstra captura a intensidade da preferência: a conexão forte da Carla (peso 10) gera um caminho de custo bem menor que a conexão fraca do Bruno (peso 1) |

Essa diferença mostra que o BFS é adequado quando todas as interações têm peso equivalente ou quando peso não está disponível, enquanto o Dijkstra é mais preciso quando a intensidade da preferência (quantidade comprada, nota de avaliação) é um sinal relevante para a recomendação — ao custo de uma estrutura de dados um pouco mais complexa (fila de prioridade) e maior custo computacional por nó explorado.

**Nota sobre empates**: quando dois produtos empatam em score no BFS, a ordem entre eles no resultado pode variar entre execuções do programa. Isso ocorre porque o `HashMap` padrão do Rust randomiza sua ordem de iteração a cada execução (medida de segurança contra ataques que exploram ordem previsível). O algoritmo atual não define um critério de desempate secundário — uma melhoria futura seria usar o id do produto ou outro critério como desempate determinístico.

## O Índice Invertido: otimizando a busca

Na primeira versão do algoritmo, "descobrir quem mais comprou um produto" exigia percorrer **todos os clientes** do grafo e verificar, para cada um, se ele havia comprado aquele produto — uma operação O(N) por produto expandido durante a busca.

Para resolver isso, o `Grafo` mantém um segundo mapa, `indice_produto_clientes: HashMap<u32, Vec<u32>>`, atualizado a cada chamada de `adicionar_compra` junto com a lista de arestas original. Esse índice guarda, pronto, "produto → lista de clientes que o compraram", transformando a busca de O(N) para O(K), onde K é apenas a quantidade de clientes que realmente compraram aquele produto — tipicamente uma fração pequena de N.

Esse é um exemplo clássico de **trade-off entre tempo e espaço**: o sistema passa a usar aproximadamente o dobro de memória (guardando a relação cliente↔produto nos dois sentidos), em troca de uma redução drástica no tempo de busca — mensurada na seção seguinte.

## Resultados de Desempenho

Medições feitas com o módulo `benchmark`, gerando dados fictícios em escalas crescentes (clientes e produtos crescendo juntos, para refletir o "catálogo com milhões de produtos" citado no edital) e medindo o tempo da função `recomendar` (BFS limitado a 3 saltos).

**Antes do índice invertido** (busca linear O(N) por todos os clientes), modo `--release`:

| Clientes / Produtos | Tempo |
|---|---|
| 1.000.000 / 1.000.000 | 283 ms |
| 2.000.000 / 2.000.000 | 306 ms |

**Depois do índice invertido** (busca O(K) via `indice_produto_clientes`), modo `--release`:

| Clientes / Produtos | Tempo |
|---|---|
| 100 / 50 | 0 ms |
| 1.000 / 200 | 0 ms |
| 10.000 / 1.000 | 0 ms |
| 100.000 / 100.000 | 0 ms |
| 1.000.000 / 1.000.000 | 0 ms |
| 5.000.000 / 5.000.000 | 0 ms |

**Limite real encontrado**: em uma escala de 10.000.000 de clientes/produtos, o processo foi encerrado pelo sistema operacional (falta de memória RAM disponível no ambiente de testes) — não um erro do algoritmo, mas o reflexo direto do trade-off tempo/espaço descrito acima: o índice invertido dobra o uso de memória para eliminar a busca linear, e em volumes muito altos esse custo de memória se torna o fator limitante antes mesmo do tempo de CPU. Por esse motivo, o benchmark do repositório foi fixado até 5.000.000, escala em que o sistema se manteve estável e rápido.

**Observações gerais:**

- A diferença entre modo `debug` e `--release` é significativa, evidenciando o custo das otimizações do compilador Rust. Importante: os **resultados** das recomendações são idênticos entre os dois modos — só o tempo de execução muda, nunca a corretude do algoritmo.
- O ganho do índice invertido foi confirmado na prática, não apenas em teoria: o mesmo cenário de 1 a 2 milhões de clientes/produtos, que levava ~300ms antes, passou a rodar em tempo não mensurável (0ms) depois da otimização.

## Testes

O projeto conta com **7 testes automatizados**:
- **5 testes unitários** (`src/grafo/mod.rs` e `src/grafo/busca.rs`, dentro de módulos `#[cfg(test)]`): validam a criação de arestas, a atualização do índice invertido, consulta de produtos existentes/inexistentes, e a recomendação a partir de produto.
- **2 testes de integração** (`tests/recomendacao.rs`): validam o fluxo completo de recomendação a partir de cliente, incluindo o caminho feliz e o caso negativo (anti-duplicata).
