use std::env;
use dotenv::dotenv;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use crate::logs::log_event;
use crate::clave_embebida::obtener_clave_embebida;

fn log_event_internal(mensaje: &str) {
    let _ = log_event(&format!("CONFIG: {}", mensaje));
}

pub fn inicializar_clave_sistema() -> Result<(), String> {
    let clave = obtener_clave_embebida();
    
    if clave.len() != 32 {
        let msg = format!("Clave embebida invalida: longitud {} bytes (debe ser 32)", clave.len());
        log_event_internal(&msg);
        return Err(msg);
    }
    
    log_event_internal("Clave criptografica inicializada desde codigo fuente embebido");
    Ok(())
}

pub fn obtener_clave_crypto() -> Option<Vec<u8>> {
    let clave = obtener_clave_embebida();
    
    if clave.len() != 32 {
        log_event_internal(&format!("Clave embebida con longitud incorrecta: {} bytes", clave.len()));
        return None;
    }
    
    Some(clave.to_vec())
}

pub fn get_db_config() -> (String, String, String, String, String) {
    dotenv().ok();
    let host = env::var("DB_HOST").unwrap_or_else(|_| "localhost".to_string());
    let port = env::var("DB_PORT").unwrap_or_else(|_| "5432".to_string());
    let database = env::var("DB_NAME").unwrap_or_else(|_| "monedas_db".to_string());
    let user = env::var("DB_USER").unwrap_or_else(|_| "postgres".to_string());
    let password = env::var("DB_PASSWORD").unwrap_or_else(|_| "".to_string());

    (host, port, database, user, password)
}

pub fn verificar_configuracion_postgres() -> (bool, Vec<String>) {
    dotenv().ok();
    let (host, _port, database, user, password) = get_db_config();
    let mut errores = Vec::new();

    if host.is_empty() {
        errores.push("DB_HOST no configurado".to_string());
    }

    if database.is_empty() {
        errores.push("DB_NAME no configurado".to_string());
    }

    if user.is_empty() {
        errores.push("DB_USER no configurado".to_string());
    }

    if password.is_empty() {
        errores.push("DB_PASSWORD no configurado (puede estar vacio si no requiere contrasena)".to_string());
    }

    (errores.is_empty(), errores)
}