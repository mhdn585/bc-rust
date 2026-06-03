use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use chrono::Local;
use serde::{Serialize, Deserialize};

const LOG_FILE: &str = "sistema.log";
const MAX_LOG_SIZE: u64 = 10 * 1024 * 1024;
const MAX_LOG_BACKUPS: usize = 5;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum NivelLog {
    INFO,
    ERROR,
}

impl NivelLog {
    pub fn as_str(&self) -> &'static str {
        match self {
            NivelLog::INFO => "INFO",
            NivelLog::ERROR => "ERROR",
        }
    }
}

pub fn obtener_timestamp() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string()
}

fn get_log_path() -> PathBuf {
    std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")).join(LOG_FILE)
}

fn get_backup_path(numero: usize) -> PathBuf {
    std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
        .join(format!("{}.{}", LOG_FILE, numero))
}

fn rotar_log_si_necesario() -> bool {
    let log_path = get_log_path();

    if !log_path.exists() {
        return false;
    }

    let tamano = match fs::metadata(&log_path) {
        Ok(meta) => meta.len(),
        Err(_) => return false,
    };

    if tamano > MAX_LOG_SIZE {
        for i in (1..MAX_LOG_BACKUPS).rev() {
            let viejo = get_backup_path(i);
            let nuevo = get_backup_path(i + 1);
            if viejo.exists() {
                let _ = fs::rename(&viejo, &nuevo);
            }
        }

        let backup_1 = get_backup_path(1);
        if backup_1.exists() {
            let _ = fs::remove_file(&backup_1);
        }

        let _ = fs::rename(&log_path, &backup_1);

        let _ = escribir_log_raw("LOG ROTADO EN ".to_string() + &obtener_timestamp());
        return true;
    }

    false
}

fn escribir_log_raw(mensaje: String) -> Result<(), std::io::Error> {
    let log_path = get_log_path();
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_path)?;

    writeln!(file, "{}", mensaje)?;
    Ok(())
}

fn escribir_log(mensaje: &str, nivel: NivelLog) {
    let _ = rotar_log_si_necesario();

    let timestamp = obtener_timestamp();
    let log_linea = format!("[{}] {}: {}", timestamp, nivel.as_str(), mensaje);
    let _ = escribir_log_raw(log_linea);
}

pub fn log_event(mensaje: &str) -> Result<(), std::io::Error> {
    escribir_log(mensaje, NivelLog::INFO);
    Ok(())
}

pub fn log_error(mensaje: &str) {
    escribir_log(mensaje, NivelLog::ERROR);
}

pub fn obtener_ultimos_logs(cantidad: usize) -> Vec<String> {
    let log_path = get_log_path();

    if !log_path.exists() {
        return Vec::new();
    }

    let contenido = match fs::read_to_string(&log_path) {
        Ok(c) => c,
        Err(_) => return Vec::new(),
    };

    let lineas: Vec<String> = contenido.lines().map(|l| l.to_string()).collect();

    let start = if lineas.len() > cantidad {
        lineas.len() - cantidad
    } else {
        0
    };

    lineas[start..].to_vec()
}

pub fn limpiar_archivo_log() -> bool {
    let log_path = get_log_path();
    if log_path.exists() {
        fs::remove_file(&log_path).is_ok()
    } else {
        true
    }
}

pub fn obtener_tamano_log() -> u64 {
    let log_path = get_log_path();
    if log_path.exists() {
        fs::metadata(&log_path).map(|m| m.len()).unwrap_or(0)
    } else {
        0
    }
}