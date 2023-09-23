//Declare uma struct chamada Retangulo que represente as dimensões de um retângulo com campos para largura e altura.
//Implemente um método que calcule e retorne a área do retângulo.
// Escreva um programa que utilize a struct e o método para calcular e imprimir a área de vários retângulos.

struct Retangulo {
    largura: f32,
    altura: f32,
}

impl Retangulo {
    fn area(&self) -> f32 {
        self.altura * self.largura / 2.0
    }
}

fn main() {
    let rect_1 = Retangulo {
        largura: 5.0,
        altura: 8.0,
    };

    let rect_2 = Retangulo {
        largura: 3.5,
        altura: 8.0,
    };

    println!("{}", rect_1.area());

    println!("{}", rect_2.area());
}
