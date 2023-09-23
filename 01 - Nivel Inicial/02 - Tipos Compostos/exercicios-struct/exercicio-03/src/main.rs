// Crie uma struct chamada ContaBancaria com campos para armazenar o nome do titular, saldo e número da conta.
// Implemente métodos para depositar e sacar dinheiro da conta, atualizando o saldo corretamente.
// Escreva um programa que demonstre o funcionamento da struct e seus métodos.

struct ContaBancaria {
    nome: String,
    saldo: i64,
    numero_conta: i32,
}

impl ContaBancaria {
    fn depositar(&mut self, valor: i64) {
        self.saldo += valor;
        println!("Saldo Atualizado!")
    }

    fn sacar(&mut self, valor: i64) {
        if self.saldo >= (self.saldo - valor) {
            self.saldo -= valor;
            println!(
                "Você sacou: {}, Saldo Atualizado!, Saldo atual: {}",
                valor, self.saldo
            );
        } else {
            println!("Saldo insufiente");
        }
    }
    fn mostrar_informacoes(&self) {
        println!(
            "Nome: {}, Saldo: {}, Número da conta: {}",
            self.nome, self.saldo, self.numero_conta
        )
    }
}

fn main() {
    let mut conta_1 = ContaBancaria {
        nome: String::from("Lucas"),
        saldo: 1000,
        numero_conta: 0001,
    };

    conta_1.mostrar_informacoes();
    conta_1.depositar(2000);

    conta_1.mostrar_informacoes();

    conta_1.sacar(500);
}
