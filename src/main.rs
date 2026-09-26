// src/main.rs

use megastore::benchmark;
use megastore::grafo::dijkstra;
use megastore::grafo::{self, Grafo};
use megastore::modelo;

use modelo::cliente::Cliente;
use modelo::produto::Produto;

fn processar_produto(produto: &Produto) {
    println!("Processando produto...");
    produto.exibir();
}

fn processar_cliente(cliente: &Cliente) {
    println!("Processando cliente...");
    cliente.exibir();
}

fn main() {
    let produto = Produto::new(1, "Fone Bluetooth", "Eletrônicos", 129.90);

    processar_produto(&produto);

    produto.exibir();

    let cliente = Cliente::new(1, "Ana Souza", "Goiânia");
    processar_cliente(&cliente);
    cliente.exibir();

    let mut g = Grafo::new();

    // Produtos
    g.adicionar_produto(modelo::produto::Produto::new(
        1,
        "Fone Bluetooth",
        "Eletrônicos",
        129.90,
    ));
    g.adicionar_produto(modelo::produto::Produto::new(
        2,
        "Carregador",
        "Eletrônicos",
        39.90,
    ));
    g.adicionar_produto(modelo::produto::Produto::new(
        3,
        "Mouse",
        "Eletrônicos",
        59.90,
    ));

    // Clientes
    g.adicionar_cliente(modelo::cliente::Cliente::new(1, "Ana Souza", "Goiânia"));
    g.adicionar_cliente(modelo::cliente::Cliente::new(2, "Bruno Lima", "Anápolis"));

    // Compras (cliente_id, produto_id, peso)
    g.adicionar_compra(1, 1, 3.0); // Ana comprou Fone
    g.adicionar_compra(1, 2, 2.0); // Ana comprou Carregador
    g.adicionar_compra(2, 1, 4.0); // Bruno comprou Fone
    g.adicionar_compra(2, 3, 1.0); // Bruno comprou Mouse

    // Recomendação para Ana (cliente 1), até 3 saltos
    let recomendacoes = grafo::busca::recomendar(&g, 1, 3);
    println!("Recomendações para Ana: {:?}", recomendacoes);

    benchmark::rodar_benchmark();

    let mut g2 = Grafo::new();

    g2.adicionar_produto(Produto::new(1, "Fone Bluetooth", "Eletrônicos", 129.90));
    g2.adicionar_produto(Produto::new(3, "Mouse", "Eletrônicos", 59.90));
    g2.adicionar_produto(Produto::new(4, "Teclado", "Eletrônicos", 149.90));

    g2.adicionar_cliente(Cliente::new(1, "Ana Souza", "Goiânia"));
    g2.adicionar_cliente(Cliente::new(2, "Bruno Lima", "Anápolis"));
    g2.adicionar_cliente(Cliente::new(3, "Carla Reis", "Aparecida de Goiânia"));

    g2.adicionar_compra(1, 1, 5.0); // Ana comprou Fone
    g2.adicionar_compra(2, 1, 1.0); // Bruno: conexão FRACA com o Fone
    g2.adicionar_compra(2, 3, 1.0); // Bruno comprou Mouse
    g2.adicionar_compra(3, 1, 10.0); // Carla: conexão FORTE com o Fone
    g2.adicionar_compra(3, 4, 1.0); // Carla comprou Teclado

    let rec_bfs = grafo::busca::recomendar(&g2, 1, 3);
    let rec_dijkstra = dijkstra::recomendar_dijkstra(&g2, 1, 5.0);

    match g.consultar_produto(1) {
        Some(produto) => produto.exibir(),
        None => println!("Produto não encontrado"),
    }

    let rec_por_produto = grafo::busca::recomendar_por_produto(&g2, 1, 2);
    println!("Quem comprou o Fone também comprou: {:?}", rec_por_produto);

    println!("\n=== Comparação BFS vs Dijkstra ===");
    println!("BFS (contagem de coocorrência): {:?}", rec_bfs);
    println!("Dijkstra (peso da conexão): {:?}", rec_dijkstra);
}
