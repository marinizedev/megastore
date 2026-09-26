// src/benchmark/mod.rs

use crate::grafo::{self, Grafo};
use crate::modelo::cliente::Cliente;
use crate::modelo::produto::Produto;
use std::time::Instant;

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
        g.adicionar_cliente(Cliente::new(
            cid,
            &format!("Cliente {}", cid),
            "Cidade Genérica",
        ));

        for i in 0..compras_por_cliente {
            let produto_id = ((cid + i) % n_produtos) + 1;
            g.adicionar_compra(cid, produto_id, 1.0);
        }
    }

    g
}

pub fn medir_tempo_recomendacao(
    grafo: &Grafo,
    cliente_id: u32,
    max_saltos: u32,
    repeticoes: u32,
) -> (Vec<(u32, u32)>, u128) {
    // Execução de aquecimento. Ela não entra na média.
    let _ = grafo::busca::recomendar(grafo, cliente_id, max_saltos);

    let mut tempo_total_nanos: u128 = 0;
    let mut ultimo_resultado: Vec<(u32, u32)> = Vec::new();

    for _ in 0..repeticoes {
        let inicio = Instant::now();

        ultimo_resultado = grafo::busca::recomendar(grafo, cliente_id, max_saltos);

        tempo_total_nanos += inicio.elapsed().as_nanos();
    }

    let tempo_medio_nanos = tempo_total_nanos / repeticoes as u128;

    (ultimo_resultado, tempo_medio_nanos)
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
        let repeticoes = 10;

        let (_resultado, tempo_medio_nanos) = medir_tempo_recomendacao(&g, 1, 3, repeticoes);

        println!(
            "{} clientes / {} produtos → média de {} ns em {} repetições",
            n_clientes, n_produtos, tempo_medio_nanos, repeticoes
        );
    }
}
