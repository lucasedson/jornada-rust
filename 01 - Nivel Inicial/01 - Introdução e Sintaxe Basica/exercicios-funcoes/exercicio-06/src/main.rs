// Soma de Números Pares: Implemente uma função soma_pares que recebe
// um número inteiro positivo n como parâmetro
//  e retorna a soma dos primeiros n números pares.

fn soma_pares(n: i32) -> i32 {
    let mut acc: i32 = 0;
    for i in 0..(n + 1) {
        if i % 2 == 0 {
            acc = acc + i;
        }
    }
    return acc;
}

fn main() {
    println!("{}", soma_pares(10000));
}
