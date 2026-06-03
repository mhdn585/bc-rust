use std::fs;
use std::path::PathBuf;
use crate::logs::{log_error, log_event};
use crate::utils::{print_verde, print_rojo, print_amarillo, print_blanco, print_azul};
use crate::db::reiniciar_base_datos;

pub fn eliminar_archivo_log() -> bool {
    let log_file = PathBuf::from("sistema.log");
    if log_file.exists() {
        match fs::remove_file(&log_file) {
            Ok(_) => {
                let _ = log_event("Archivo sistema.log eliminado");
                true
            }
            Err(e) => {
                log_error(&format!("Error al eliminar sistema.log: {}", e));
                false
            }
        }
    } else {
        true
    }
}

pub async fn eliminar_clave_crypto() -> bool {
    let project_root = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let key_path = project_root.join("crypto_key.key");

    let mut exito = true;

    if key_path.exists() {
        if let Err(e) = fs::remove_file(&key_path) {
            log_error(&format!("Error al eliminar clave crypto: {}", e));
            exito = false;
        }
    }

    exito
}

pub async fn reiniciar_sistema_postgres(_crear_backup: bool, limpiar_logs: bool, conservar_clave: bool) -> bool {
    let _ = log_event("INICIANDO REINICIO DEL SISTEMA CON POSTGRESQL");

    if limpiar_logs {
        eliminar_archivo_log();
    }

    if !conservar_clave {
        eliminar_clave_crypto().await;
        print_azul("Clave criptografica eliminada (se regenerara al iniciar)");
    }

    print_amarillo("Eliminando todos los datos de PostgreSQL...");
    let exito = reiniciar_base_datos().await;

    if limpiar_logs {
        let _ = log_event("Logs eliminados durante reinicio");
    }

    if exito {
        let _ = log_event("REINICIO COMPLETADO EXITOSAMENTE");
        print_verde("Sistema reiniciado correctamente");
        print_azul("Ejecuta 'generar' para crear nuevas monedas");
        true
    } else {
        log_error("Error critico durante reinicio");
        print_rojo("Error al reiniciar el sistema");
        false
    }
}