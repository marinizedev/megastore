# ConectaStore — Sistema de Recomendação de Produtos Baseado em Grafos

O **ConectaStore** é um sistema de recomendação de produtos desenvolvido para a **MegaStore**, uma empresa fictícia de comércio eletrônico apresentada no desafio da disciplina. O projeto utiliza grafos e estruturas de dados complementares para gerar recomendações com base nas relações entre clientes, compras e produtos.

Desenvolvido em **Rust** para a disciplina **Data Structure Strategy and Implementation**, da UniFECAF, o sistema busca substituir recomendações genéricas — como produtos mais vendidos ou itens da mesma categoria — por recomendações baseadas no comportamento de compra dos clientes.

A implementação utiliza apenas recursos da biblioteca padrão do Rust, incluindo estruturas de dados e algoritmos de percurso em grafos.

## Índice

- [Objetivo e funcionamento](#objetivo-e-funcionamento)
- [Modelagem como grafo](#modelagem-como-grafo)
- [Tecnologias e estruturas de dados](#tecnologias-e-estruturas-de-dados)
- [Arquitetura do projeto](#arquitetura-do-projeto)
- [Compilação, execução e testes](#compilação-execução-e-testes)
- [Exemplos de uso](#exemplos-de-uso)
- [Algoritmo BFS](#algoritmo-bfs)
- [Algoritmo de Dijkstra](#algoritmo-de-dijkstra)
- [BFS versus Dijkstra](#bfs-versus-dijkstra)
- [Índice invertido](#índice-invertido)
- [Resultados de desempenho](#resultados-de-desempenho)
- [Testes automatizados](#testes-automatizados)
- [Limitações e melhorias futuras](#limitações-e-melhorias-futuras)
- [Licença](#licença)

## Objetivo e funcionamento

A MegaStore é um e-commerce fictício com um catálogo muito grande de produtos. Com o crescimento do catálogo, recomendações baseadas apenas em produtos mais vendidos ou na mesma categoria podem ser genéricas e pouco relevantes para cada cliente.

O problema central do projeto é:

> Como utilizar grafos e estruturas de dados complementares para gerar recomendações relevantes em um catálogo com grande volume de produtos?

O ConectaStore representa as relações de compra entre clientes e produtos em um grafo. A partir dessas relações, o sistema consegue identificar clientes que possuem comportamentos semelhantes e recomendar produtos que ainda não foram comprados pelo cliente de origem.

O sistema oferece dois tipos principais de recomendação:

1. **Recomendação a partir de um cliente:** identifica outros clientes que compraram produtos em comum e sugere produtos adicionais comprados por esses clientes semelhantes.
2. **Recomendação a partir de um produto:** identifica quem comprou determinado produto e quais outros produtos essas pessoas também compraram.

## Modelagem como grafo

O sistema utiliza um **grafo bipartido**, formado por dois conjuntos distintos de vértices:

- clientes;
- produtos.

A relação principal é representada por uma compra:

```text
Cliente → Produto
```

### Vértices

Cada cliente possui:

- `id` numérico;
- nome;
- cidade.

Cada produto possui:

- `id` numérico;
- nome;
- categoria;
- preço.

### Arestas

Cada aresta representa uma compra entre um cliente e um produto. A aresta armazena um peso numérico, que pode representar, por exemplo, a quantidade comprada ou a intensidade da preferência do cliente.

### Direção semântica e índice reverso

A direção semântica da compra é **Cliente → Produto**. Entretanto, para realizar recomendações colaborativas, o sistema também mantém uma estrutura auxiliar no sentido inverso:

```text
Produto → Clientes que compraram
```

Esse índice reverso não altera o significado da compra. Ele apenas permite consultar de maneira eficiente quais clientes compraram determinado produto, evitando uma varredura sobre todos os clientes cadastrados.

### Tipo do grafo

O grafo é:

- **bipartido**, porque possui clientes e produtos como conjuntos distintos;
- **ponderado**, porque as compras possuem pesos;
- semanticamente direcionado de Cliente para Produto;
- consultável nos dois sentidos por meio da lista de adjacência e do índice invertido.

A similaridade entre dois clientes ou dois produtos não é armazenada como uma aresta direta. Ela é inferida durante o percurso do grafo. Por exemplo, dois clientes são considerados semelhantes quando compraram um ou mais produtos em comum.

## Tecnologias e estruturas de dados

- **Linguagem:** Rust;
- **Edição do Rust:** 2024;
- **Dependências externas:** nenhuma; o projeto utiliza apenas a biblioteca padrão;
- **Gerenciador:** Cargo.

| Estrutura | Utilização | Justificativa |
|---|---|---|
| `HashMap<u32, Produto>` | Cadastro e consulta de produtos por ID | Permite acesso médio O(1) por identificador. |
| `HashMap<u32, Cliente>` | Cadastro e consulta de clientes por ID | Permite acesso médio O(1) por identificador. |
| `HashMap<u32, Vec<(u32, f64)>>` | Lista de adjacência Cliente → Produto | Representa apenas as relações existentes, adequada a um grafo esparso. |
| `HashMap<u32, Vec<u32>>` | Índice invertido Produto → Cliente | Evita percorrer todos os clientes para descobrir quem comprou determinado produto. |
| `VecDeque` | Fila do BFS | Permite inserção no final e remoção no início em O(1). |
| `HashSet` | Controle de clientes e produtos visitados | Evita ciclos, reprocessamento e recomendações de itens já comprados. |
| `BinaryHeap` | Fila de prioridade do Dijkstra | Permite processar os estados pelo menor custo acumulado. |
| `HashMap<u32, u32>` | Pontuação das recomendações BFS | Acumula quantos clientes semelhantes levaram a cada produto candidato. |

A lista de adjacência foi escolhida em vez de uma matriz de adjacência porque o grafo é esparso. Uma matriz teria custo O(V²), mesmo quando a maior parte das possíveis relações entre clientes e produtos não existisse.

## Arquitetura do projeto

```text
megastore/
├── Cargo.toml
├── Cargo.lock
├── README.md
├── .gitignore
├── src/
│   ├── lib.rs
│   ├── main.rs
│   ├── modelo/
│   │   ├── mod.rs
│   │   ├── cliente.rs
│   │   └── produto.rs
│   ├── grafo/
│   │   ├── mod.rs
│   │   ├── busca.rs
│   │   └── dijkstra.rs
│   └── benchmark/
│       └── mod.rs
└── tests/
    └── recomendacao.rs
```

### Responsabilidade dos módulos

- `modelo/cliente.rs`: define a estrutura `Cliente`;
- `modelo/produto.rs`: define a estrutura `Produto`;
- `grafo/mod.rs`: define o grafo, os cadastros, as arestas e o índice invertido;
- `grafo/busca.rs`: implementa as recomendações por BFS;
- `grafo/dijkstra.rs`: implementa a recomendação ponderada por Dijkstra;
- `benchmark/mod.rs`: gera dados fictícios e mede o tempo de recomendação;
- `main.rs`: executa demonstrações dos recursos do sistema;
- `tests/recomendacao.rs`: contém os testes de integração.

O tipo `Grafo` mantém quatro estruturas principais:

```rust
pub produtos: HashMap<u32, Produto>
pub clientes: HashMap<u32, Cliente>
pub arestas: HashMap<u32, Vec<(u32, f64)>>
pub indice_produto_clientes: HashMap<u32, Vec<u32>>
```

## Compilação, execução e testes

### Pré-requisito

É necessário ter Rust e Cargo instalados. A instalação oficial está disponível em:

<https://www.rust-lang.org/tools/install>

### Clonar o repositório

Clone o repositório público para sua máquina usando:

```bash
git clone https://github.com/marinizedev/megastore.git
```

Depois, entre na pasta do projeto:

```bash
cd megastore
```

Após entrar na pasta `megastore`, os comandos `cargo` apresentados nas seções seguintes podem ser executados normalmente.

Para conferir se os arquivos principais foram clonados corretamente:

```bash
ls
```

A raiz do projeto deverá conter, entre outros arquivos:

```text
Cargo.toml
Cargo.lock
README.md
src/
tests/
```

### Compilar e executar em modo de desenvolvimento

```bash
cargo run
```

Esse comando compila e executa o programa principal, que apresenta exemplos de cadastro, recomendação, comparação entre BFS e Dijkstra e benchmark.

### Executar em modo otimizado

```bash
cargo run --release
```

O modo `--release` é recomendado para as medições de desempenho, pois utiliza as otimizações do compilador Rust.

### Executar todos os testes

```bash
cargo test --all-targets
```

### Verificar a formatação

```bash
cargo fmt -- --check
```

Para formatar automaticamente os arquivos:

```bash
cargo fmt
```

### Executar o Clippy

```bash
cargo clippy --all-targets -- -D warnings
```

O projeto foi ajustado para passar nessa verificação sem warnings.

## Exemplos de uso

### Cadastro de produtos, clientes e compras

```rust
let mut g = Grafo::new();

g.adicionar_produto(Produto::new(
    1,
    "Fone Bluetooth",
    "Eletrônicos",
    129.90,
));

g.adicionar_produto(Produto::new(
    2,
    "Carregador",
    "Eletrônicos",
    39.90,
));

g.adicionar_produto(Produto::new(
    3,
    "Mouse",
    "Eletrônicos",
    59.90,
));

g.adicionar_cliente(Cliente::new(
    1,
    "Ana Souza",
    "Goiânia",
));

g.adicionar_cliente(Cliente::new(
    2,
    "Bruno Lima",
    "Anápolis",
));

g.adicionar_compra(1, 1, 3.0); // Ana comprou o Fone
g.adicionar_compra(1, 2, 2.0); // Ana comprou o Carregador
g.adicionar_compra(2, 1, 4.0); // Bruno comprou o Fone
g.adicionar_compra(2, 3, 1.0); // Bruno comprou o Mouse
```

### Consulta de produto

```rust
if let Some(produto) = g.consultar_produto(1) {
    produto.exibir();
}
```

### Recomendação a partir de um cliente

```rust
let recomendacoes = grafo::busca::recomendar(&g, 1, 3);
println!("Recomendações: {:?}", recomendacoes);
```

Nesse exemplo, Ana e Bruno compraram o mesmo Fone Bluetooth. Como Bruno também comprou o Mouse, o sistema recomenda o Mouse para Ana. O Carregador não é recomendado novamente porque Ana já o comprou.

Resultado esperado:

```text
[(3, 1)]
```

O primeiro valor é o ID do produto e o segundo é o score da recomendação.

### Recomendação a partir de um produto

```rust
let relacionados = grafo::busca::recomendar_por_produto(&g, 1, 2);
println!("Produtos relacionados: {:?}", relacionados);
```

Essa consulta responde à pergunta:

> Quem comprou o Fone Bluetooth também comprou quais produtos?

Quando vários clientes compraram o mesmo produto relacionado, o score é incrementado para cada cliente. Assim, produtos comprados por mais clientes tendem a aparecer com maior pontuação.

## Algoritmo BFS

A Busca em Largura é o algoritmo principal do sistema. Ela percorre o grafo em camadas, usando uma `VecDeque`.

A recomendação a partir de um cliente segue esta lógica:

```text
Cliente de origem
       ↓
Produtos comprados pelo cliente
       ↓
Outros clientes que compraram esses produtos
       ↓
Produtos comprados pelos clientes semelhantes
```

O percurso é limitado por `max_saltos` para evitar explorar o grafo inteiro desnecessariamente.

A pontuação é calculada por coocorrência:

- cada cliente semelhante que comprou um produto candidato adiciona um ponto;
- produtos que o cliente de origem já comprou são excluídos;
- produtos duplicados não aparecem repetidamente no resultado;
- empates são resolvidos pelo ID do produto, garantindo uma saída determinística.

A BFS não utiliza o peso das compras para calcular a pontuação. Portanto, uma conexão com peso 1 e uma conexão com peso 10 contribuem igualmente para o score da BFS.

## Algoritmo de Dijkstra

O projeto também implementa Dijkstra como alternativa ponderada à BFS.

O custo da aresta é calculado como:

```text
custo = 1.0 / peso_da_compra
```

Consequentemente:

- pesos maiores produzem custos menores;
- conexões mais fortes são priorizadas;
- o caminho de menor custo acumulado é processado primeiro;
- a `BinaryHeap` é utilizada como fila de prioridade.

O parâmetro `max_distancia` limita o custo máximo permitido. Estados e produtos cujo custo ultrapassa esse valor não são incluídos no resultado.

A implementação pressupõe pesos positivos. Pesos zero ou negativos não representam uma intensidade válida para o cálculo inverso utilizado pelo Dijkstra.

## BFS versus Dijkstra

Considere o seguinte cenário:

- Ana comprou o Fone;
- Bruno comprou o Fone com peso 1 e também comprou o Mouse;
- Carla comprou o Fone com peso 10 e também comprou o Teclado.

A BFS considera apenas a existência das relações e pode gerar empate:

| Algoritmo | Resultado | Interpretação |
|---|---|---|
| BFS | Mouse e Teclado com score 1 | Os dois produtos foram encontrados por um cliente semelhante. O peso não é considerado. |
| Dijkstra | Produto associado à conexão forte primeiro | A conexão de peso 10 gera menor custo que a conexão de peso 1. |

A BFS é mais simples e adequada quando o peso não está disponível ou quando todas as relações têm importância equivalente. O Dijkstra é mais apropriado quando a intensidade da interação deve influenciar a ordem das recomendações.

## Índice invertido

Na primeira abordagem, para descobrir quais clientes compraram determinado produto seria necessário percorrer todos os clientes e verificar suas listas de compras.

Essa busca poderia custar O(N) por produto consultado, em que N é o número de clientes.

Para otimizar essa operação, o projeto mantém o índice:

```rust
HashMap<u32, Vec<u32>>
```

Esse mapa associa:

```text
produto_id → clientes que compraram o produto
```

Com o índice, a consulta passa a depender da quantidade K de clientes que realmente compraram o produto:

```text
Antes: O(N)
Depois: O(K)
```

Essa otimização representa um trade-off entre tempo e espaço. O sistema armazena a relação Cliente–Produto nos dois sentidos, consumindo mais memória, mas reduzindo o custo de busca.

## Resultados de desempenho

O módulo `benchmark` gera dados fictícios em escalas crescentes e mede o tempo da função de recomendação BFS limitada a três saltos.

Os dados são gerados com:

- quantidades crescentes de clientes;
- quantidades crescentes de produtos;
- cinco compras por cliente;
- execução em modo `--release` recomendada;
- uma execução de aquecimento, que não entra na média;
- 10 repetições da recomendação por cenário;
- apresentação da média dos tempos em nanossegundos.

Exemplo de execução:

```bash
cargo run --release
```

A medição atual utiliza `Instant` e apresenta a média dos tempos em nanossegundos (`ns`). O benchmark executa cada cenário 10 vezes, depois de uma execução de aquecimento que não entra no cálculo da média.

Exemplo de saída:

```text
=== Benchmark de desempenho ===
100 clientes / 50 produtos → média de 10367 ns em 10 repetições
1000 clientes / 200 produtos → média de 37948 ns em 10 repetições
10000 clientes / 1000 produtos → média de 39756 ns em 10 repetições
100000 clientes / 100000 produtos → média de 3827 ns em 10 repetições
1000000 clientes / 1000000 produtos → média de 3446 ns em 10 repetições
5000000 clientes / 5000000 produtos → média de 3772 ns em 10 repetições
```

Os valores acima são um exemplo de uma execução em modo `--release`. Eles podem variar conforme o hardware, o sistema operacional, a carga do ambiente e o modo de compilação. O modo `--release` tende a ser mais rápido por utilizar as otimizações do compilador.

As medições não incluem a geração do grafo; elas medem especificamente a chamada da função de recomendação. Como a BFS é limitada a três saltos e explora apenas a vizinhança alcançada, o tempo medido não cresce necessariamente de forma linear com o tamanho total do cadastro.

Como melhoria futura, o benchmark pode calcular também mediana e desvio-padrão, além de registrar automaticamente informações do hardware e do sistema operacional usados no teste.

### Complexidade

Considerando:

- V = quantidade de vértices;
- A = quantidade de arestas;
- S = limite de saltos;
- K = quantidade de clientes associados a determinado produto.

A BFS com lista de adjacência e índice invertido possui, no pior caso, complexidade O(V + A). Como o percurso é limitado por saltos, o custo real pode ser menor, dependendo da densidade da região explorada.

O Dijkstra com `BinaryHeap` possui complexidade aproximada de:

```text
O((V + A) log V)
```

A representação do grafo utiliza espaço linear:

```text
O(V + A)
```

Com o índice invertido, o espaço total continua linear em relação à entrada, aproximadamente:

```text
O(V + 2A)
```

Isso é muito menor que O(V²), custo de uma matriz de adjacência completa.

## Testes automatizados

O projeto possui **18 testes automatizados**, sendo 16 testes unitários e 2 testes de integração:

### Testes unitários

São executados dentro dos módulos do código e verificam:

- criação correta de arestas;
- atualização do índice invertido;
- consulta de produto existente;
- consulta de produto inexistente;
- recomendação por produto;
- soma do score quando vários clientes compraram o mesmo produto relacionado;
- respeito ao limite de distância do Dijkstra.
- rejeição de pesos zero, negativos, `NaN` e infinito;
- aceitação de peso positivo e finito;
- comportamento de cliente sem compras;
- comportamento de produto sem compradores;
- comportamento com zero saltos;
- comportamento do Dijkstra para cliente sem compras.

### Testes de integração

O arquivo `tests/recomendacao.rs` verifica:

- recomendação de produto comprado por cliente semelhante;
- prevenção de recomendação de produtos que o cliente já possui.

Para executar todos:

```bash
cargo test --all-targets
```

Resultado da validação atual:

```text
16 testes unitários aprovados
2 testes de integração aprovados
18 testes aprovados no total
```

Também foram utilizados os comandos de qualidade:

```bash
cargo fmt -- --check
cargo clippy --all-targets -- -D warnings
```

## Limitações e melhorias futuras

Apesar de atender aos requisitos principais do trabalho, algumas melhorias podem ser implementadas em versões futuras:

- calcular também mediana e desvio-padrão no benchmark;
- registrar automaticamente informações do hardware e do sistema operacional usado no benchmark;
- tratar compras duplicadas de forma explícita, atualizando o peso ou rejeitando a duplicação;
- retornar `Result` ao cadastrar compras inválidas, em vez de apenas retornar um valor booleano;
- adicionar uma interface de entrada para cadastro de dados reais;
- persistir clientes, produtos e compras em arquivos ou banco de dados;
- incorporar categorias, avaliações e outros sinais ao cálculo da recomendação;
- comparar o desempenho com outras estratégias de recomendação;
- adicionar testes para IDs de clientes e produtos inexistentes;
- avaliar estratégias adicionais de desempate e relevância.

## Repositório

Repositório público do projeto:

<https://github.com/marinizedev/megastore>

## Licença

Projeto acadêmico desenvolvido para fins educacionais.
