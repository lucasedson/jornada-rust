# Tipos Compostos

Em Rust, os tipos compostos permitem que você agrupe múltiplos valores de diferentes tipos em uma única unidade. Existem quatro tipos compostos principais em Rust: `tuplas`, `arrays`, `enums` e `structs`. Todos desempenham um papel importante na organização e manipulação de dados mais complexos.

## Tuples (Tuplas):
Uma tuple é uma coleção ordenada de elementos com tipos diferentes. Cada elemento pode ter um tipo distinto. As tuplas têm um tamanho fixo e são usadas para agrupar valores relacionados. Elas são declaradas entre parênteses e os elementos são separados por vírgulas. As tuplas são úteis quando você precisa combinar diferentes tipos de dados em uma única estrutura.

Exemplo de uma tuple:

```rust
fn main() {
    let tupla: (i32, f64, char) = (42, 3.14, 'A');
    let primeiro_elemento = tupla.0; // Acesso ao primeiro elemento (42)
    let segundo_elemento = tupla.1;  // Acesso ao segundo elemento (3.14)
    let terceiro_elemento = tupla.2; // Acesso ao terceiro elemento ('A')

    println!("Elementos da tupla: {}, {}, {}", primeiro_elemento, segundo_elemento, terceiro_elemento);
}

```

## Arrays:
Um array é uma coleção de elementos do mesmo tipo, com tamanho fixo em tempo de compilação. Os elementos em um array são acessados por índices inteiros. Ao contrário das tuplas, todos os elementos de um array têm o mesmo tipo. Os arrays são mais eficientes em termos de memória e velocidade de acesso, mas menos flexíveis em comparação com as coleções dinâmicas, como os vetores.

Exemplo de um array:

```rust
fn main() {
    let numeros = [1, 2, 3, 4, 5];
    let primeiro_elemento = numeros[0]; // Acesso ao primeiro elemento (1)
    let segundo_elemento = numeros[1];  // Acesso ao segundo elemento (2)

    println!("Elementos do array: {}, {}", primeiro_elemento, segundo_elemento);
}

```
## Structs (Estruturas):
As structs permitem que você defina seus próprios tipos de dados personalizados agrupando campos com diferentes tipos. Você pode criar estruturas que representam entidades do seu domínio. Cada instância de uma struct é um objeto com campos nomeados. As structs fornecem mais clareza e organização ao código, permitindo que você modele dados complexos de maneira mais significativa.

Exemplo de uma struct:
```rust
struct Pessoa {
    nome: String,
    idade: u32,
}

fn main() {
    let pessoa1 = Pessoa {
        nome: String::from("Alice"),
        idade: 30,
    };

    println!("Nome: {}, Idade: {}", pessoa1.nome, pessoa1.idade);
}

```
## Enums (Enumerados):
Os enums são tipos que podem ter um de vários valores possíveis, chamados de "variantes". Cada variante pode ter dados associados ou não. Os enums são frequentemente usados para modelar escolhas ou estados diferentes. Eles ajudam a garantir que os valores sejam representados de forma segura e completa.

Exemplo de um enum:

```rust
enum Resultado {
    Sucesso(u32),
    Erro(String),
}

fn main() {
    let resposta = Resultado::Sucesso(42);

    match resposta {
        Resultado::Sucesso(valor) => println!("Sucesso: {}", valor),
        Resultado::Erro(msg) => println!("Erro: {}", msg),
    }
}
```

## `impl` (Implementação):

Em Rust, impl é uma palavra-chave usada para implementar métodos e comportamentos para tipos personalizados, como structs e enums. Ela permite que você defina a implementação de um trait (ou características específicas) para um determinado tipo. Essa funcionalidade é fundamental para a programação orientada a objetos em Rust e para adicionar comportamentos personalizados aos seus próprios tipos de dados.

A sintaxe básica da implementação (impl) em Rust é a seguinte:

```rust
impl NomeDoTipo {
    // Métodos e comportamentos a serem implementados para o tipo
}
```
Aqui estão alguns exemplos de como o impl é usado:

### Exemplo com struct:

```rust
struct Retangulo {
    largura: u32,
    altura: u32,
}

impl Retangulo {
    fn area(&self) -> u32 {
        self.largura * self.altura
    }
}

fn main() {
    let retangulo = Retangulo { largura: 10, altura: 20 };
    println!("Área do retângulo: {}", retangulo.area());
}
```

### Exemplo com enum:
```rust
enum Moeda {
    Real(u32),
    Dolar(f64),
}

impl Moeda {
    fn converter_para_real(&self) -> f64 {
        match *self {
            Moeda::Real(valor) => valor as f64,
            Moeda::Dolar(valor) => valor * 5.3,
        }
    }
}

fn main() {
    let real = Moeda::Real(100);
    let dolar = Moeda::Dolar(10.0);

    println!("{} reais é igual a {:.2} dólares", real.converter_para_real(), dolar.converter_para_real());
}
```

O `impl` permite que você defina funções e métodos específicos para tipos. Isso ajuda a organizar o código, melhorar a legibilidade e adicionar funcionalidades personalizadas aos seus tipos de dados.