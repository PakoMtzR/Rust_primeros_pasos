use rand::Rng;
use std::io;

fn main() {
    println!("Adivina el numero\n--------------------");

    let mut str_max_num = String::new();
    println!("Ingresa el numero maximo para el juego:");
    io::stdin()
        .read_line(&mut str_max_num)
        .expect("fallo en el ingreso");

    // Leemos la entrada (limite: numero maximo del juego)
    let int_max_num: u32 = str_max_num.trim().parse().unwrap();

    // Creamos el numero secreto
    let secret_num: u32 = rand::thread_rng().gen_range(0..int_max_num);

    let mut tries: u32 = 5;
    loop {
        println!(
            "Tienes {} intentos, adivina el numero [0-{}]",
            tries, str_max_num
        );
        let mut str_num: String = String::new();
        io::stdin().read_line(&mut str_num).expect("fallo...");
        let int_num: u32 = str_num.trim().parse().unwrap();

        if int_num == secret_num {
            println!("Adivinaste");
            break;
        } else {
            tries -= 1;
            println!("No, intenta nuevamente.");
        }
        if tries == 0 {
            println!("Has perdido");
            break;
        }
    }
}
