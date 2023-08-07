//Fatorial: Escreva uma função fatorial que calcula o fatorial de um número inteiro positivo usando recursão.
//NOTA: Não usei recursão pq não é uma boa Prática
fn main() {
    fn fatorial(numero: i128) -> i128 {
        let mut acc: i128 = numero;
        for i in (1..(numero)).rev() {
            acc = acc * i;
        }
        return acc;
    }
    let num: i128 = 5;
    println!("O Fatorial de {} é: {}", num, fatorial(num));
}
