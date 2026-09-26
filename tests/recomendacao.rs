// tests/recomendacao.rs

use megastore::grafo::{self, Grafo};
use megastore::modelo::produto::Produto;
use megastore::modelo::cliente::Cliente;

#[test]
fn recomenda_produto_de_cliente_parecido() {
    let mut g = Grafo::new();

    g.adicionar_produto(Produto::new(1, "Fone Bluetooth", "Eletrônicos", 129.90));
    g.adicionar_produto(Produto::new(2, "Carregador", "Eletrônicos", 39.90));
    g.adicionar_produto(Produto::new(3, "Mouse", "Eletrônicos", 59.90));

    g.adicionar_cliente(Cliente::new(1, "Ana Souza", "Goiânia"));
    g.adicionar_cliente(Cliente::new(2, "Bruno Lima", "Anápolis"));

    g.adicionar_compra(1, 1, 3.0);
    g.adicionar_compra(1, 2, 2.0);
    g.adicionar_compra(2, 1, 4.0);
    g.adicionar_compra(2, 3, 1.0);

    let recomendacoes = grafo::busca::recomendar(&g, 1, 3);

    assert_eq!(recomendacoes, vec![(3, 1)]);
}

#[test]
fn nao_recomenda_produto_que_cliente_ja_possui() {
    let mut g = Grafo::new();

    g.adicionar_produto(Produto::new(1, "Fone Bluetooth", "Eletrônicos", 129.90));
    g.adicionar_produto(Produto::new(2, "Carregador", "Eletrônicos", 39.90));

    g.adicionar_cliente(Cliente::new(1, "Ana Souza", "Goiânia"));
    g.adicionar_cliente(Cliente::new(2, "Bruno Lima", "Anápolis"));

    g.adicionar_compra(1, 1, 3.0);
    g.adicionar_compra(1, 2, 2.0);
    g.adicionar_compra(2, 1, 4.0);
    g.adicionar_compra(2, 2, 1.0); // Bruno também já tem o Carregador

    let recomendacoes = grafo::busca::recomendar(&g, 1, 3);

    // Ana já tem os dois produtos que Bruno tem, então não deve sobrar nada pra recomendar
    assert!(recomendacoes.is_empty());
}