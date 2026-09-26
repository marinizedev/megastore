// src/benchmark/mod.rs

use std::time::Instant;
use crate::grafo::{self, Grafo};
use crate::modelo::produto::Produto;
use crate::modelo::cliente::Cliente;

pub fn gerar_dados_ficticios(n_clientes: u32, n_produtos: u32, compras_por_cliente: u32) -> Grafo {
    let mut g = Grafo::new();

    for pid in 1..=n_produtos {
        g.adicionar_produto(Produto::new(
            pid,
            &format!("Produto {}", pid),
            "Categoria Genérica",
            9.90,
        ));
    }

    for cid in 1..=n_clientes {
        g.adicionar_cliente(Cliente::new(cid, &format!("Cliente {}", cid), "Cidade Genérica"));

        for i in 0..compras_por_cliente {
            let produto_id = ((cid + i) % n_produtos) + 1;
            g.adicionar_compra(cid, produto_id, 1.0);
        }
    }

    g
}

pub fn medir_tempo_recomendacao(grafo: &Grafo, cliente_id: u32, max_saltos: u32) -> (Vec<(u32, u32)>, u128) {
    let inicio = Instant::now();
    let resultado = grafo::busca::recomendar(grafo, cliente_id, max_saltos);
    let duracao = inicio.elapsed().as_millis();

    (resultado, duracao)
}

pub fn rodar_benchmark() {
    let escalas = [
        (100, 50),
        (1_000, 200),
        (10_000, 1_000),
        (100_000, 100_000),
        (1_000_000, 1_000_000),
        (5_000_000, 5_000_000),
    ];

    println!("\n=== Benchmark de desempenho ===");
    for (n_clientes, n_produtos) in escalas {
        let g = gerar_dados_ficticios(n_clientes, n_produtos, 5);
        let (_resultado, duracao_ms) = medir_tempo_recomendacao(&g, 1, 3);

        println!(
            "{} clientes / {} produtos → recomendação em {} ms",
            n_clientes, n_produtos, duracao_ms
        );
    }
}