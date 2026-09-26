// src/modelo/cliente.rs

#[derive(Debug)]
pub struct Cliente {
    pub id: u32,
    pub nome: String,
    pub cidade: String,
}

impl Cliente {
    pub fn new(id: u32, nome: &str, cidade: &str) -> Cliente {
        Cliente {
            id,
            nome: nome.to_string(),
            cidade: cidade.to_string(),
        }
    }

    pub fn exibir(&self) {
        println!("[{}] {} — {}", self.id, self.nome, self.cidade);
    }
}