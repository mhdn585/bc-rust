use std::env;
use dotenv::dotenv;
use crate::logs::log_event;
use crate::clave_embebida::obtener_clave_embebida;

pub const TOTAL_MONEDAS: i64 = 100_000;

pub const TIEMPO_MINADO_SEGUNDOS: u64 = 360;

pub const MIN_TIEMPO_MINADO: u64 = 60;
pub const MAX_TIEMPO_MINADO: u64 = 15_552_000;

fn log_event_internal(mensaje: &str) {
    let _ = log_event(&format!("CONFIG: {}", mensaje));
}

pub fn validar_tiempo_minado(tiempo: u64) -> u64 {
    if tiempo < MIN_TIEMPO_MINADO {
        log_event_internal(&format!("Tiempo {}s es menor que el minimo {}s, usando minimo", tiempo, MIN_TIEMPO_MINADO));
        return MIN_TIEMPO_MINADO;
    }
    if tiempo > MAX_TIEMPO_MINADO {
        log_event_internal(&format!("Tiempo {}s es mayor que el maximo {}s, usando maximo", tiempo, MAX_TIEMPO_MINADO));
        return MAX_TIEMPO_MINADO;
    }
    tiempo
}

pub fn obtener_tiempo_minado() -> u64 {
    let tiempo_validado = validar_tiempo_minado(TIEMPO_MINADO_SEGUNDOS);
    log_event_internal(&format!("Tiempo de minado configurado: {} segundos por moneda completa", tiempo_validado));
    tiempo_validado
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