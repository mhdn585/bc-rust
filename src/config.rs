use std::env;
use std::fs;
use std::path::PathBuf;
use dotenv::dotenv;
use rand::RngCore;
use rand::rngs::OsRng;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use crate::logs::log_event;

const KEY_FILE: &str = "crypto_key.key";

pub fn get_project_root() -> PathBuf {
    match std::env::current_exe() {
        Ok(exe_path) => {
            exe_path.parent().unwrap_or(&PathBuf::from(".")).to_path_buf()
        }
        Err(_) => PathBuf::from(".")
    }
}

fn log_event_internal(mensaje: &str) {
    let _ = log_event(&format!("CONFIG: {}", mensaje));
}

pub fn generar_clave_aes() -> Option<Vec<u8>> {
    let mut clave = vec![0u8; 32];
    match OsRng.try_fill_bytes(&mut clave) {
        Ok(_) => Some(clave),
        Err(e) => {
            let _ = log_event_internal(&format!("Error al generar clave AES: {}", e));
            None
        }
    }
}

pub fn guardar_clave(clave: &[u8]) -> bool {
    let project_root = get_project_root();
    let key_path = project_root.join(KEY_FILE);

    match fs::write(&key_path, clave) {
        Ok(_) => {
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                if let Ok(metadata) = fs::metadata(&key_path) {
                    let mut permissions = metadata.permissions();
                    permissions.set_mode(0o400);
                    let _ = fs::set_permissions(&key_path, permissions);
                }
            }
            let _ = log_event_internal(&format!("Clave guardada en {}", key_path.display()));
            true
        }
        Err(e) => {
            let _ = log_event_internal(&format!("Error al guardar clave: {}", e));
            false
        }
    }
}

pub fn cargar_clave() -> Option<Vec<u8>> {
    let project_root = get_project_root();
    let key_path = project_root.join(KEY_FILE);

    if !key_path.exists() {
        return None;
    }

    match fs::read(&key_path) {
        Ok(clave) => {
            if clave.len() != 32 {
                let _ = log_event_internal(&format!("Clave con longitud incorrecta: {} bytes", clave.len()));
                return None;
            }
            Some(clave)
        }
        Err(e) => {
            let _ = log_event_internal(&format!("Error al cargar clave: {}", e));
            None
        }
    }
}

pub fn obtener_clave_crypto() -> Option<Vec<u8>> {
    if let Ok(clave_env) = env::var("CRYPTO_KEY") {
        if let Ok(clave) = STANDARD.decode(&clave_env) {
            if clave.len() == 32 {
                let _ = log_event_internal("Clave cargada desde variable de entorno");
                return Some(clave);
            }
        }
    }

    if let Some(clave_archivo) = cargar_clave() {
        let _ = log_event_internal("Clave cargada desde archivo");
        return Some(clave_archivo);
    }

    let _ = log_event_internal("Generando nueva clave AES-256...");
    if let Some(nueva_clave) = generar_clave_aes() {
        if guardar_clave(&nueva_clave) {
            let _ = log_event_internal("Nueva clave generada y guardada exitosamente");
        } else {
            let _ = log_event_internal("Clave generada pero no se pudo guardar");
        }
        return Some(nueva_clave);
    }

    let _ = log_event_internal("Error critico al obtener clave crypto");
    None
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

pub fn eliminar_clave_crypto() -> bool {
    let project_root = get_project_root();
    let key_path = project_root.join(KEY_FILE);

    if key_path.exists() {
        let _ = fs::remove_file(&key_path);
        let _ = log_event_internal("Clave crypto eliminada");
    }

    true
}

pub fn regenerar_clave_crypto() -> Option<Vec<u8>> {
    eliminar_clave_crypto();
    if let Some(nueva_clave) = generar_clave_aes() {
        if guardar_clave(&nueva_clave) {
            let _ = log_event_internal("Clave crypto regenerada exitosamente");
            return Some(nueva_clave);
        }
    }
    let _ = log_event_internal("Error al regenerar clave crypto");
    None
}

pub fn cargar_modo_color() -> bool {
    let config_path = get_project_root().join("color_mode.config");
    if config_path.exists() {
        if let Ok(contenido) = fs::read_to_string(&config_path) {
            let modo = contenido.trim().to_lowercase();
            if modo == "false" {
                crate::utils::set_color_mode(false);
                return false;
            }
        }
    }
    crate::utils::set_color_mode(true);
    true
}

pub fn guardar_modo_color(modo: bool) -> bool {
    let config_path = get_project_root().join("color_mode.config");
    let contenido = if modo { "true" } else { "false" };
    match fs::write(&config_path, contenido) {
        Ok(_) => true,
        Err(_) => false
    }
}

pub fn get_color_mode() -> bool {
    crate::utils::get_color_mode()
}

pub fn set_color_mode(modo: bool) {
    crate::utils::set_color_mode(modo);
}