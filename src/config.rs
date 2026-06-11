use std::env;
use dotenv::dotenv;
use crate::logs::log_event;
use crate::clave_embebida::obtener_clave_embebida;

#[allow(dead_code)]
pub const TOTAL_MONEDAS: i64 = 2_000_000;
#[allow(dead_code)]
pub const MONEDAS_POR_TABLA: i64 = 100_000;
#[allow(dead_code)]
pub const TOTAL_TABLAS: i64 = TOTAL_MONEDAS / MONEDAS_POR_TABLA;

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

#[allow(dead_code)]
pub fn obtener_nombre_tabla(id_moneda_global: i64) -> String {
    let numero_tabla = ((id_moneda_global - 1) / MONEDAS_POR_TABLA) % TOTAL_TABLAS;
    format!("monedas_{:02}", numero_tabla)
}

#[allow(dead_code)]
pub fn obtener_todas_las_tablas() -> Vec<String> {
    let mut tablas = Vec::with_capacity(TOTAL_TABLAS as usize);
    for i in 0..TOTAL_TABLAS {
        tablas.push(format!("monedas_{:02}", i));
    }
    tablas
}