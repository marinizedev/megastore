// src/modelo/produto.rs

#[derive(Debug)]
pub struct Produto {
    pub id: u32,
    pub nome: String,
    pub categoria: String,
    pub preco: f64,
}

impl Produto {
    pub fn new(id: u32, nome: &str, categoria: &str, preco: f64) -> Produto {
        Produto {
            id,
            nome: nome.to_string(),
            categoria: categoria.to_string(),
            preco,
        }
    }

    pub fn exibir(&self) {
        println!(
            "[{}] {} — {} — R$ {:.2}",
            self.id, self.nome, self.categoria, self.preco
        );
    }
}