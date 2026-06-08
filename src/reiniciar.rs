use std::fs;
use std::path::PathBuf;
use crate::logs::{log_error, log_event};
use crate::utils::{print_verde, print_rojo, print_amarillo, print_azul};
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

pub async fn reiniciar_sistema_postgres(_crear_backup: bool, limpiar_logs: bool, _conservar_clave: bool) -> bool {
    let _ = log_event("INICIANDO REINICIO DEL SISTEMA CON POSTGRESQL");

    if limpiar_logs {
        eliminar_archivo_log();
    }

    print_azul("La clave criptografica permanece embebida en el sistema");

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