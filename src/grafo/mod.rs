// src/grafo/mod.rs

pub mod busca;
pub mod dijkstra;

use crate::modelo::cliente::Cliente;
use crate::modelo::produto::Produto;
use std::collections::HashMap;

pub struct Grafo {
    pub produtos: HashMap<u32, Produto>,
    pub clientes: HashMap<u32, Cliente>,
    pub arestas: HashMap<u32, Vec<(u32, f64)>>,
    pub indice_produto_clientes: HashMap<u32, Vec<u32>>,
}

impl Grafo {
    pub fn new() -> Grafo {
        Grafo {
            produtos: HashMap::new(),
            clientes: HashMap::new(),
            arestas: HashMap::new(),
            indice_produto_clientes: HashMap::new(),
        }
    }

    pub fn adicionar_produto(&mut self, produto: Produto) {
        self.produtos.insert(produto.id, produto);
    }

    pub fn adicionar_cliente(&mut self, cliente: Cliente) {
        self.clientes.insert(cliente.id, cliente);
    }

    pub fn adicionar_compra(&mut self, cliente_id: u32, produto_id: u32, peso: f64) -> bool {
        if !peso.is_finite() || peso <= 0.0 {
            eprintln!("Compra ignorada: o peso deve ser finito e maior que zero.");

            return false;
        }

        self.arestas
            .entry(cliente_id)
            .or_default()
            .push((produto_id, peso));

        self.indice_produto_clientes
            .entry(produto_id)
            .or_default()
            .push(cliente_id);

        true
    }

    pub fn consultar_produto(&self, id: u32) -> Option<&Produto> {
        self.produtos.get(&id)
    }

    pub fn consultar_cliente(&self, id: u32) -> Option<&Cliente> {
        self.clientes.get(&id)
    }
}

impl Default for Grafo {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modelo::cliente::Cliente;
    use crate::modelo::produto::Produto;

    #[test]
    fn adicionar_compra_cria_aresta_corretamente() {
        let mut g = Grafo::new();
        g.adicionar_cliente(Cliente::new(1, "Ana", "Goiânia"));
        g.adicionar_produto(Produto::new(1, "Fone", "Eletrônicos", 100.0));
        g.adicionar_compra(1, 1, 5.0);

        assert_eq!(g.arestas.get(&1), Some(&vec![(1, 5.0)]));
    }

    #[test]
    fn consultar_produto_existente_retorna_some() {
        let mut g = Grafo::new();
        g.adicionar_produto(Produto::new(1, "Fone", "Eletrônicos", 100.0));

        assert!(g.consultar_produto(1).is_some());
    }

    #[test]
    fn consultar_produto_inexistente_retorna_none() {
        let g = Grafo::new();
        assert!(g.consultar_produto(999).is_none());
    }

    #[test]
    fn adicionar_compra_atualiza_indice_invertido() {
        let mut g = Grafo::new();
        g.adicionar_cliente(Cliente::new(1, "Ana", "Goiânia"));
        g.adicionar_produto(Produto::new(1, "Fone", "Eletrônicos", 100.0));
        g.adicionar_compra(1, 1, 5.0);

        assert_eq!(g.indice_produto_clientes.get(&1), Some(&vec![1]));
    }

    #[test]
    fn rejeita_peso_zero() {
        let mut g = Grafo::new();

        let resultado = g.adicionar_compra(1, 1, 0.0);

        assert!(!resultado);
        assert!(g.arestas.is_empty());
        assert!(g.indice_produto_clientes.is_empty());
    }

    #[test]
    fn rejeita_peso_negativo() {
        let mut g = Grafo::new();

        let resultado = g.adicionar_compra(1, 1, -5.0);

        assert!(!resultado);
        assert!(g.arestas.is_empty());
        assert!(g.indice_produto_clientes.is_empty());
    }

    #[test]
    fn rejeita_peso_nan() {
        let mut g = Grafo::new();

        let resultado = g.adicionar_compra(1, 1, f64::NAN);

        assert!(!resultado);
        assert!(g.arestas.is_empty());
        assert!(g.indice_produto_clientes.is_empty());
    }

    #[test]
    fn rejeita_peso_infinito() {
        let mut g = Grafo::new();

        let resultado = g.adicionar_compra(1, 1, f64::INFINITY);

        assert!(!resultado);
        assert!(g.arestas.is_empty());
        assert!(g.indice_produto_clientes.is_empty());
    }

    #[test]
    fn aceita_peso_positivo_e_finito() {
        let mut g = Grafo::new();

        let resultado = g.adicionar_compra(1, 1, 3.0);

        assert!(resultado);
        assert_eq!(g.arestas.get(&1), Some(&vec![(1, 3.0)]));
        assert_eq!(g.indice_produto_clientes.get(&1), Some(&vec![1]));
    }
}
