use std::io::{self, Write};

pub const LOG_FILE: &str = "sistema.log";

pub fn print_blanco(texto: &str) {
    println!("\x1b[97m{}\x1b[0m", texto);
}

pub fn print_verde(texto: &str) {
    println!("\x1b[92m{}\x1b[0m", texto);
}

pub fn print_rojo(texto: &str) {
    println!("\x1b[91m{}\x1b[0m", texto);
}

pub fn print_amarillo(texto: &str) {
    println!("\x1b[93m{}\x1b[0m", texto);
}

pub fn print_azul(texto: &str) {
    println!("\x1b[94m{}\x1b[0m", texto);
}

pub fn print_morado(texto: &str) {
    println!("\x1b[95m{}\x1b[0m", texto);
}

pub fn print_cyan(texto: &str) {
    println!("\x1b[96m{}\x1b[0m", texto);
}

pub fn input_filtrado(mensaje: &str) -> String {
    print!("{}", mensaje);
    io::stdout().flush().unwrap();
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).unwrap();
    entrada.trim().to_string()
}

pub fn limpiar_pantalla() {
    if cfg!(windows) {
        let _ = std::process::Command::new("cmd").arg("/c").arg("cls").status();
    } else {
        let _ = std::process::Command::new("clear").status();
    }
}

pub fn obtener_timestamp() -> String {
    chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}