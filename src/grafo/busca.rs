// src/grafo/busca.rs

use std::collections::{HashMap, HashSet, VecDeque};
use crate::grafo::Grafo;

pub fn recomendar(grafo: &Grafo, cliente_id: u32, max_saltos: u32) -> Vec<(u32, u32)> {
    let mut visitados_clientes: HashSet<u32> = HashSet::new();
    let mut visitados_produtos: HashSet<u32> = HashSet::new();
    let mut scores: HashMap<u32, u32> = HashMap::new();

    let mut fila: VecDeque<(u32, u32, bool)> = VecDeque::new();
    fila.push_back((cliente_id, 0, true));
    visitados_clientes.insert(cliente_id);

    let produtos_do_cliente: HashSet<u32> = grafo
        .arestas
        .get(&cliente_id)
        .map(|lista| lista.iter().map(|(pid, _)| *pid).collect())
        .unwrap_or_default();

    while let Some((atual_id, salto, eh_cliente)) = fila.pop_front() {
        if salto >= max_saltos {
            continue;
        }

        if eh_cliente {
            if let Some(compras) = grafo.arestas.get(&atual_id) {
                for (produto_id, _peso) in compras {
                    if visitados_produtos.insert(*produto_id) {
                        fila.push_back((*produto_id, salto + 1, false));
                    }
                }
            }
        } else {
            if let Some(clientes_que_compraram) = grafo.indice_produto_clientes.get(&atual_id) {
                for outro_cliente_id in clientes_que_compraram {
                    if visitados_clientes.insert(*outro_cliente_id) {
                        fila.push_back((*outro_cliente_id, salto + 1, true));

                        if let Some(compras_dele) = grafo.arestas.get(outro_cliente_id) {
                            for (produto_id, _) in compras_dele {
                                if !produtos_do_cliente.contains(produto_id) {
                                    *scores.entry(*produto_id).or_insert(0) += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let mut resultado: Vec<(u32, u32)> = scores.into_iter().collect();
    resultado.sort_by(|a, b| b.1.cmp(&a.1));
    resultado
}

pub fn recomendar_por_produto(grafo: &Grafo, produto_id: u32, max_saltos: u32) -> Vec<(u32, u32)> {
    let mut visitados_clientes: HashSet<u32> = HashSet::new();
    let mut visitados_produtos: HashSet<u32> = HashSet::new();
    let mut scores: HashMap<u32, u32> = HashMap::new();

    let mut fila: VecDeque<(u32, u32, bool)> = VecDeque::new();
    fila.push_back((produto_id, 0, false));
    visitados_produtos.insert(produto_id);

    while let Some((atual_id, salto, eh_cliente)) = fila.pop_front() {
        if salto >= max_saltos {
            continue;
        }

        if eh_cliente {
            if let Some(compras) = grafo.arestas.get(&atual_id) {
                for (pid, _peso) in compras {
                    if *pid != produto_id && visitados_produtos.insert(*pid) {
                        fila.push_back((*pid, salto + 1, false));
                        *scores.entry(*pid).or_insert(0) += 1;
                    }
                }
            }
        } else {
            if let Some(clientes_que_compraram) = grafo.indice_produto_clientes.get(&atual_id) {
                for cliente_id in clientes_que_compraram {
                    if visitados_clientes.insert(*cliente_id) {
                        fila.push_back((*cliente_id, salto + 1, true));
                    }
                }
            }
        }
    }

    let mut resultado: Vec<(u32, u32)> = scores.into_iter().collect();
    resultado.sort_by(|a, b| b.1.cmp(&a.1));
    resultado
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modelo::produto::Produto;
    use crate::modelo::cliente::Cliente;

    #[test]
    fn recomendar_por_produto_encontra_produtos_relacionados() {
        let mut g = Grafo::new();
        g.adicionar_produto(Produto::new(1, "Fone", "Eletrônicos", 100.0));
        g.adicionar_produto(Produto::new(2, "Mouse", "Eletrônicos", 50.0));
        g.adicionar_cliente(Cliente::new(1, "Ana", "Goiânia"));
        g.adicionar_compra(1, 1, 3.0);
        g.adicionar_compra(1, 2, 2.0);

        let resultado = recomendar_por_produto(&g, 1, 2);

        assert_eq!(resultado, vec![(2, 1)]);
    }
}