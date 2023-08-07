fn main() {
    fn verificar_paridade(number: i32) -> String {
        if number % 2 == 0 {
            return "Par".to_string();
        } else {
            return "Impar".to_string();
        }
    }

    println!("{}", verificar_paridade(13));
    println!("{}", verificar_paridade(12));
}
