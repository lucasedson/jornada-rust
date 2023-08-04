# "Propriedade" (Ownership) e "Empréstimo" (Borrowing)

"Propriedade" (**Ownership**) e "Empréstimo" (**Borrowing**) são conceitos fundamentais na linguagem de programação Rust, projetados para garantir a segurança de memória, evitar problemas de concorrência e facilitar a gerência de recursos. Esses conceitos ajudam a prevenir erros como vazamentos de memória, condições de corrida e acesso inválido à memória, que são comuns em outras linguagens de programação.

## Propriedade (Ownership):
A propriedade em Rust se refere ao conceito de que cada valor em Rust tem uma "dona" claramente definida. Um valor pode ter apenas uma única propriedade e, quando essa propriedade é transferida (por exemplo, quando uma variável é atribuída a outra), o valor original não pode mais ser usado. Isso evita problemas como liberar a mesma memória duas vezes ou acessar memória que foi desalocada.

A propriedade também está relacionada à gestão automática de memória em Rust. Quando uma propriedade é perdida (por exemplo, quando a variável que a detém sai de escopo), Rust chama automaticamente a função drop, que é responsável por liberar quaisquer recursos associados (como memória alocada dinamicamente).

A propriedade em Rust refere-se à ideia de que cada valor em Rust tem uma única "dona" (variável ou estrutura) que é responsável por liberar o valor quando ele não for mais necessário. Quando um valor é atribuído a outra variável, a propriedade é transferida, e a variável original não pode mais acessar o valor.

Exemplo:

```rust
fn main() {
    let s1 = String::from("Olá"); // s1 é a dona da String
    let s2 = s1; // s1 transfere a propriedade para s2
    // Aqui, s1 não pode mais ser usada, pois a propriedade foi transferida para s2
    println!("{}", s2); // Isso é válido
}

```


## Empréstimo (Borrowing):
Em vez de transferir a propriedade, Rust permite emprestar temporariamente uma referência a um valor para outra parte do código. Isso é conhecido como "empréstimo" ou "referência". Existem duas formas principais de empréstimo: referência mutável e referência imutável.

Referência Mutável: Permite que você altere o valor através da referência. No entanto, Rust garante que, em um determinado escopo, você só pode ter uma referência mutável a um valor.

Referência Imutável: Permite que você leia o valor através da referência, mas não modificá-lo. Rust permite várias referências imutáveis simultâneas a um valor, garantindo que não haja acesso concorrente de escrita.

Os empréstimos ajudam a evitar condições de corrida, onde vários threads ou partes do código tentam modificar os mesmos dados simultaneamente.

O empréstimo em Rust permite que você compartilhe temporariamente uma referência a um valor sem transferir a propriedade. Isso é feito através de referências, que podem ser imutáveis (você pode ler o valor) ou mutáveis (você pode ler e escrever o valor, mas apenas um empréstimo mutável é permitido em um determinado escopo).

O empréstimo em Rust permite que você compartilhe temporariamente uma referência a um valor sem transferir a propriedade. Isso é feito através de referências, que podem ser imutáveis (você pode ler o valor) ou mutáveis (você pode ler e escrever o valor, mas apenas um empréstimo mutável é permitido em um determinado escopo).

Exemplo:
```rust
fn main() {
    let s1 = String::from("Olá"); // s1 é a dona da String

    // Empréstimo imutável
    let len = calcular_comprimento(&s1); // Passamos uma referência imutável
    println!("Comprimento: {}", len);

    // Empréstimo mutável
    let mut s2 = String::from("Mundo");
    alterar_string(&mut s2); // Passamos uma referência mutável
    println!("{}", s2);
}

fn calcular_comprimento(s: &String) -> usize {
    s.len() // Não precisamos de propriedade, apenas empréstimo imutável
}

fn alterar_string(s: &mut String) {
    s.push_str(" Maravilhoso!"); // Podemos modificar o valor através do empréstimo mutável
}

```


## Resumo:
A propriedade e o empréstimo são conceitos essenciais em Rust para garantir a segurança de memória e evitar erros comuns de programação. Essas características combinadas permitem que você crie código seguro, concorrente e eficiente, sem a necessidade explícita de um coletor de lixo ou outras técnicas de gerenciamento de memória complexas. Isso torna Rust uma linguagem poderosa para sistemas e aplicações de alto desempenho.