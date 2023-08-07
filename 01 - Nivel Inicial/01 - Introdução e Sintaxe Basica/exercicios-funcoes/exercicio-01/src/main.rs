fn main() {
    fn calc(a: f64, b: f64, op: &str) -> f64 {
        if op == "plus" {
            return a + b;
        } else if op == "multiply" {
            return a * b;
        } else if op == "minus" {
            return a - b;
        } else {
            return a / b;
        }
    }
    println!("{}", calc(5.0, 4.0, "plus"));
    println!("{}", calc(5.0, 4.0, "multiply"));
    println!("{}", calc(5.0, 4.0, "minus"));
    println!("{}", calc(5.0, 4.0, "divide"));
}
