// Exercício: Cadastro de Pessoas
// Crie uma struct chamada Pessoa que represente uma pessoa com campos como nome, idade, cidade e e-mail.
// Escreva um programa que crie várias instâncias da struct e imprima as informações de cada pessoa.

struct Pessoa {
    nome: String,
    idade: i16,
    cidade: String,
    email: String,
}

fn nova_pessoa(pessoa: Pessoa) -> Pessoa {
    println!(
        "Nome: {}\nIdade: {}\nCidade: {}\nEmail: {}",
        pessoa.nome, pessoa.idade, pessoa.cidade, pessoa.email
    );
    return pessoa;
}

fn main() {
    let pessoa_1 = Pessoa {
        nome: String::from("Lucas"),
        idade: 27,
        cidade: String::from("Rio de Janeiro"),
        email: String::from("email@email.com"),
    };

    nova_pessoa(pessoa_1);
}
