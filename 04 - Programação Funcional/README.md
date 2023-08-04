# Programação Funcional

A programação funcional em Rust é um paradigma de programação que enfatiza o uso de funções como blocos de construção fundamentais e trata os programas como avaliações de expressões matemáticas. Embora Rust seja uma linguagem multiparadigma, ela oferece suporte a muitos dos princípios da programação funcional, permitindo que você escreva código mais conciso, legível e seguro.

## Closures em Rust:
Uma closure é uma função anônima que pode capturar variáveis do ambiente em que foi definida. Em Rust, closures são expressas usando a sintaxe de |parâmetros| expressão. Elas podem ser usadas para criar funções flexíveis e genéricas em situações onde uma função definida separadamente não é necessária.

Exemplo de closure em Rust:

```rust
fn main() {
    let add = |x, y| x + y;
    let result = add(3, 5);
    println!("Resultado: {}", result);
}
```
## Map, Filter e Reduce:

Map: O método map é usado para aplicar uma função a cada elemento de uma sequência (como um vetor ou um iterador) e retornar uma nova sequência com os resultados.
Exemplo de uso de map em Rust:

```rust

fn main() {
    let numeros = vec![1, 2, 3, 4, 5];
    let quadrados: Vec<i32> = numeros.iter().map(|x| x * x).collect();
    println!("Quadrados: {:?}", quadrados);
}
```
Filter: O método filter é usado para criar uma nova sequência contendo apenas os elementos que atendem a um certo critério, determinado por uma função.
Exemplo de uso de filter em Rust:

```rust

fn main() {
    let numeros = vec![1, 2, 3, 4, 5];
    let pares: Vec<i32> = numeros.iter().filter(|x| x % 2 == 0).cloned().collect();
    println!("Pares: {:?}", pares);
}
```
Reduce (Fold): O método fold é usado para acumular os elementos de uma sequência em um único valor, aplicando uma função acumuladora.
Exemplo de uso de fold em Rust:

```rust

fn main() {
    let numeros = vec![1, 2, 3, 4, 5];
    let soma: i32 = numeros.iter().fold(0, |acc, x| acc + x);
    println!("Soma: {}", soma);
}
```
## Combinando Map, Filter e Reduce:
Você pode combinar essas operações para realizar transformações complexas em sequências de dados. Por exemplo, você pode filtrar elementos, aplicar uma transformação a cada elemento restante e, em seguida, reduzir os resultados a um único valor.

Exemplo combinando map, filter e fold em Rust:

```rust

fn main() {
    let numeros = vec![1, 2, 3, 4, 5];
    let resultado: i32 = numeros
        .iter()
        .filter(|x| x % 2 == 0)
        .map(|x| x * x)
        .fold(0, |acc, x| acc + x);
    println!("Resultado: {}", resultado);
}
```
A programação funcional com closures, map, filter e reduce em Rust permite que você escreva código expressivo e conciso para manipular sequências de dados de maneira poderosa. Esses conceitos facilitam a implementação de transformações e operações complexas de maneira eficaz e legível.