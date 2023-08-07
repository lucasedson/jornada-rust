fn main() {
    // Conversão de Temperatura: Crie duas funções:
    // uma para converter graus Celsius para Fahrenheit (celsius_para_fahrenheit)
    // e outra para converter Fahrenheit para Celsius (fahrenheit_para_celsius).

    fn celsius_para_fahrenheit(celsius: f32) -> f32 {
        return ((celsius * 1.8) as i32 + 32) as f32;
    }

    let celsius: f32 = 25.0;

    println!(
        "{}°C em Fahrenheit é: {}F",
        celsius,
        celsius_para_fahrenheit(celsius)
    );

    fn fahrenheit_para_celsius(fah: f32) -> f32 {
        return (fah as i32 - 32) as f32 / 1.8;
    }

    let fah: f32 = 76.9;
    println!("{}F em Celsius é: {}°C", fah, fahrenheit_para_celsius(fah));
}
