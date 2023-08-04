# Sintaxe Básica

A sintaxe básica do Rust inclui os elementos fundamentais necessários para escrever programas na linguagem.

Aqui está uma explicação detalhada dos principais aspectos da sintaxe básica do Rust:

### Comentários:
Em Rust, os comentários de linha única são denotados por //, e os comentários de várias linhas são denotados por /* ... */.

```rust
// Isto é um comentário de linha única

/*
   Isto é um comentário de várias linhas.
   Pode abranger várias linhas de texto.
*/
```

### Variáveis e Mutabilidade:
Em Rust, as variáveis são, por padrão, imutáveis. Você precisa usar a palavra-chave `mut` para torná-las mutáveis.

```rust
let x = 5;
let mut x = 10;
```

### Tipos de Dados:
Rust tem vários tipos de dados básicos, incluindo inteiros, ponto flutuante, caracteres e booleanos.

```rust
let inteiro: i32 = 42;
let ponto_flutuante: f64 = 3.14;
let caractere: char = 'A';
let booleano: bool = true;

```
### Constantes:
As constantes são definidas usando a palavra-chave `const` e devem ter um tipo explícito.
```rust
const PI: f64 = 3.14;
```

### Strings:
As strings podem ser representadas como slices de caracteres (`&str`) ou como strings alocadas dinamicamente (String).

```rust
let string: &str = "Hello, world!";
let string: String = "Hello, world!".to_string();
let name = String::from("John Doe");
```

### Estruturas de Controle:
Rust suporta estruturas de controle como if, else, while e loop.

```rust
if idade >= 18 {
    println!("Você é maior de idade.");
} else {
    println!("Você é menor de idade.");
}

let mut contador = 0;
while contador < 5 {
    println!("Contador: {}", contador);
    contador += 1;
}

for numero in 1..=5 {
    println!("Número: {}", numero);
}

```

### Funções:
As funções podem ser definidas usando a palavra-chave `fn`. O tipo de retorno é especificado após os parametros.

```rust
fn saudacao(nome: &str) {
    println!("Olá, {}!", nome);
}


fn soma(x: i32, y: i32) -> i32 {
    x + y
}
```

### Entrada e Saida:
A entrada padrão pode ser lida usando a função
`std::io::stdin()` e a saída padrão pode ser escrita usando `println!()`.

```rust
use std::io;

fn main() {
    println!("Digite seu nome:");
    let mut nome = String::new();
    io::stdin().read_line(&mut nome).expect("Erro ao ler entrada");
    println!("Olá, {}!", nome.trim());
}

```
