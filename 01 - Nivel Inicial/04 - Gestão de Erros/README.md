# Gestão de Erros

Em Rust, a gestão de erros é uma parte crucial do design da linguagem. Dois tipos importantes para tratar erros de forma eficiente são Result e Option. Vamos entender como cada um deles funciona:

**Result:**
O tipo Result<T, E> é usado para representar uma operação que pode resultar em um valor bem-sucedido do tipo T ou em um erro do tipo E. Isso é particularmente útil em funções que podem falhar e precisam indicar o motivo da falha.

- Ok(T): Representa o resultado bem-sucedido com um valor do tipo T.
- Err(E): Representa um erro com um valor do tipo E.
Exemplo de uso do Result:

```rust
fn dividir(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        return Err(String::from("Divisão por zero"));
    }
    Ok(a / b)
}

fn main() {
    let resultado = dividir(10.0, 2.0);
    match resultado {
        Ok(valor) => println!("Resultado: {}", valor),
        Err(msg) => println!("Erro: {}", msg),
    }
}

```
**Option:**
O tipo Option<T> é usado para representar um valor opcional que pode ser Some(T) (indicando que há um valor do tipo T) ou None (indicando ausência de valor).

Exemplo de uso do Option:

```rust
fn encontrar_valor(lista: Vec<i32>, valor: i32) -> Option<usize> {
    for (indice, &item) in lista.iter().enumerate() {
        if item == valor {
            return Some(indice);
        }
    }
    None
}

fn main() {
    let numeros = vec![1, 2, 3, 4, 5];
    match encontrar_valor(numeros, 3) {
        Some(indice) => println!("Valor encontrado no índice: {}", indice),
        None => println!("Valor não encontrado"),
    }
}

```

O uso de `Result` e `Option` ajuda a criar código mais seguro, expressivo e robusto em Rust, incentivando práticas de tratamento adequado de erros e valores opcionais. Ao utilizar esses tipos, você está promovendo uma abordagem mais explícita e segura para lidar com situações imprevisíveis no seu código.