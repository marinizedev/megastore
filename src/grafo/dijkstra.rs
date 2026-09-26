// src/grafo/dijkstra.rs

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

use crate::grafo::Grafo;

#[derive(Copy, Clone, PartialEq)]
struct Estado {
    custo: f64,
    id: u32,
    eh_cliente: bool,
}

impl Eq for Estado {}

impl Ord for Estado {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .custo
            .partial_cmp(&self.custo)
            .unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for Estado {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn recomendar_dijkstra(grafo: &Grafo, cliente_id: u32, max_distancia: f64) -> Vec<(u32, f64)> {
    let mut distancias: HashMap<(u32, bool), f64> = HashMap::new();
    let mut heap: BinaryHeap<Estado> = BinaryHeap::new();
    let mut melhores_produtos: HashMap<u32, f64> = HashMap::new();

    distancias.insert((cliente_id, true), 0.0);

    heap.push(Estado {
        custo: 0.0,
        id: cliente_id,
        eh_cliente: true,
    });

    let produtos_do_cliente: HashSet<u32> = grafo
        .arestas
        .get(&cliente_id)
        .map(|lista| lista.iter().map(|(pid, _)| *pid).collect())
        .unwrap_or_default();

    while let Some(Estado {
        custo,
        id,
        eh_cliente,
    }) = heap.pop()
    {
        if custo > max_distancia {
            continue;
        }

        if let Some(&melhor) = distancias.get(&(id, eh_cliente))
            && custo > melhor
        {
            continue;
        }

        if eh_cliente {
            if let Some(compras) = grafo.arestas.get(&id) {
                for &(produto_id, peso) in compras {
                    let peso_aresta = 1.0 / peso.max(0.0001);
                    let novo_custo = custo + peso_aresta;
                    let chave = (produto_id, false);

                    // Só continua o percurso se o custo estiver
                    // dentro da distância máxima permitida.
                    if novo_custo <= max_distancia
                        && novo_custo < *distancias.get(&chave).unwrap_or(&f64::INFINITY)
                    {
                        distancias.insert(chave, novo_custo);

                        heap.push(Estado {
                            custo: novo_custo,
                            id: produto_id,
                            eh_cliente: false,
                        });

                        if !produtos_do_cliente.contains(&produto_id) {
                            let atual =
                                melhores_produtos.entry(produto_id).or_insert(f64::INFINITY);

                            if novo_custo < *atual {
                                *atual = novo_custo;
                            }
                        }
                    }
                }
            }
        } else if let Some(clientes_que_compraram) = grafo.indice_produto_clientes.get(&id) {
            for &outro_cliente_id in clientes_que_compraram {
                if let Some(compras) = grafo.arestas.get(&outro_cliente_id)
                    && let Some(&(_, peso)) = compras.iter().find(|(pid, _)| *pid == id)
                {
                    let peso_aresta = 1.0 / peso.max(0.0001);
                    let novo_custo = custo + peso_aresta;
                    let chave = (outro_cliente_id, true);

                    if novo_custo <= max_distancia
                        && novo_custo < *distancias.get(&chave).unwrap_or(&f64::INFINITY)
                    {
                        distancias.insert(chave, novo_custo);

                        heap.push(Estado {
                            custo: novo_custo,
                            id: outro_cliente_id,
                            eh_cliente: true,
                        });
                    }
                }
            }
        }
    }

    let mut resultado: Vec<(u32, f64)> = melhores_produtos.into_iter().collect();

    resultado.sort_by(|a, b| {
        a.1.partial_cmp(&b.1)
            .unwrap_or(Ordering::Equal)
            .then_with(|| a.0.cmp(&b.0))
    });

    resultado
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modelo::cliente::Cliente;
    use crate::modelo::produto::Produto;

    #[test]
    fn dijkstra_nao_recomenda_produto_fora_da_distancia_maxima() {
        let mut g = Grafo::new();

        g.adicionar_produto(Produto::new(1, "Fone", "Eletrônicos", 100.0));

        g.adicionar_produto(Produto::new(2, "Mouse", "Eletrônicos", 50.0));

        g.adicionar_cliente(Cliente::new(1, "Ana", "Goiânia"));

        g.adicionar_cliente(Cliente::new(2, "Bruno", "Anápolis"));

        g.adicionar_compra(1, 1, 10.0);
        g.adicionar_compra(2, 1, 10.0);
        g.adicionar_compra(2, 2, 1.0);

        // O custo total até o Mouse será maior que 0.5.
        let resultado = recomendar_dijkstra(&g, 1, 0.5);

        assert!(
            resultado.is_empty(),
            "Produto fora do limite foi recomendado: {resultado:?}"
        );
    }

    #[test]
    fn dijkstra_cliente_sem_compras_retorna_lista_vazia() {
        let mut g = Grafo::new();

        g.adicionar_cliente(Cliente::new(1, "Ana", "Goiânia"));

        let resultado = recomendar_dijkstra(&g, 1, 10.0);

        assert!(resultado.is_empty());
    }
}
