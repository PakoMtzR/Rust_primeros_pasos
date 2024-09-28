use std::io;

// Entrada de datos
fn main() {
    println!("Ingresa: ");
    let mut input: String = String::new();

    io::stdin().read_line(&mut input).expect("fallo...");
    println!("Ingresaste: {}", input);
}
