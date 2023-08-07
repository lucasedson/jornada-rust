fn main() {
    fn calcular_media(lista: Vec<i32>) -> f32 {
        let mut sum: i32 = 0;
        for i in &lista {
            sum += i;
        }
        return (sum / lista.len() as i32) as f32;
    }

    let lista = vec![0, 1, 2, 3, 4, 5, 6];
    let res = calcular_media(lista);
    println!("A média de lista é: {}", res)
}
