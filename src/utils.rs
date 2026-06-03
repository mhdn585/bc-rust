use std::io::{self, Write};
use std::fs;
use std::path::PathBuf;
use crate::logs::log_event;

pub const LOG_FILE: &str = "sistema.log";

static mut COLOR_MODO: bool = true;

pub fn set_color_mode(modo: bool) {
    unsafe {
        COLOR_MODO = modo;
    }
}

pub fn get_color_mode() -> bool {
    unsafe {
        COLOR_MODO
    }
}

pub fn print_blanco(texto: &str) {
    if get_color_mode() {
        println!("\x1b[97m{}\x1b[0m", texto);
    } else {
        println!("{}", texto);
    }
}

pub fn print_verde(texto: &str) {
    if get_color_mode() {
        println!("\x1b[92m{}\x1b[0m", texto);
    } else {
        println!("{}", texto);
    }
}

pub fn print_rojo(texto: &str) {
    if get_color_mode() {
        println!("\x1b[91m{}\x1b[0m", texto);
    } else {
        println!("{}", texto);
    }
}

pub fn print_amarillo(texto: &str) {
    if get_color_mode() {
        println!("\x1b[93m{}\x1b[0m", texto);
    } else {
        println!("{}", texto);
    }
}

pub fn print_azul(texto: &str) {
    if get_color_mode() {
        println!("\x1b[94m{}\x1b[0m", texto);
    } else {
        println!("{}", texto);
    }
}

pub fn print_morado(texto: &str) {
    if get_color_mode() {
        println!("\x1b[95m{}\x1b[0m", texto);
    } else {
        println!("{}", texto);
    }
}

pub fn print_cyan(texto: &str) {
    if get_color_mode() {
        println!("\x1b[96m{}\x1b[0m", texto);
    } else {
        println!("{}", texto);
    }
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

pub fn mostrar_logs(cantidad: usize) {
    let log_path = PathBuf::from(LOG_FILE);

    if !log_path.exists() {
        print_rojo("No hay logs disponibles");
        return;
    }

    let contenido = match fs::read_to_string(&log_path) {
        Ok(c) => c,
        Err(_) => {
            print_rojo("Error al leer logs");
            return;
        }
    };

    let lineas: Vec<&str> = contenido.lines().collect();

    if lineas.is_empty() {
        print_amarillo("No hay logs registrados");
        return;
    }

    let start = if lineas.len() > cantidad {
        lineas.len() - cantidad
    } else {
        0
    };

    let ultimos_logs = &lineas[start..];

    print_amarillo(&format!("Ultimos {} logs", ultimos_logs.len()));
    print_blanco("-".repeat(50).as_str());

    for linea in ultimos_logs {
        if linea.contains("ERROR") {
            print_rojo(linea);
        } else if linea.contains("ADVERTENCIA") {
            print_amarillo(linea);
        } else {
            print_blanco(linea);
        }
    }

    print_blanco("-".repeat(50).as_str());
    print_blanco(&format!("Total logs: {}", lineas.len()));
}

pub fn obtener_timestamp() -> String {
    chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}