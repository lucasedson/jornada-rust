// 2. Exercício: Produtos em Estoque
// Declare uma struct chamada Produto que represente um produto com campos como nome, preço e quantidade em estoque.
// Escreva um programa que crie alguns produtos, atualize suas quantidades e imprima as informações de cada produto.

struct Produto {
    nome: String,
    preco: i32,
    qtd: i32,
}

impl Produto {
    fn atualiza_qtd(&mut self, qtd: i32) {
        self.qtd = qtd;
    }
    fn mostra_produto(&self) {
        println!(
            "\t    Nome: {}
            Preço: {}
            Quantidade: {}\n\n",
            self.nome, self.preco, self.qtd
        );
    }
}

fn main() {
    let mut produto_1 = Produto {
        nome: String::from("Caderno"),
        preco: 1900,
        qtd: 25,
    };

    produto_1.mostra_produto();

    produto_1.atualiza_qtd(24);

    produto_1.mostra_produto();
}
