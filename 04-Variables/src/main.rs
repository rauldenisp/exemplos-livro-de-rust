fn shadowing() {
    println!("\nshadowing");
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("O valor de x é: {}", x);
}
fn tipos() {
    println!("\ntipos");
    // let mut espacos = "   ";
    // espacos = espacos.len();
}

fn main() {
    println!("main");
    let mut x = 5;
    println!("O valor de x é: {}", x);
    x = 6;
    println!("O valor de x é: {}", x);
    shadowing();
    tipos();
}
