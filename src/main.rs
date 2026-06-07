use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::process;
use ctrlc;

mod config;
mod db;
mod crypto_aes;
mod logs;
mod utils;
mod crear_monedas;
mod minar;
mod reiniciar;
mod models;

use crate::config::{obtener_clave_crypto, verificar_configuracion_postgres};
use crate::db::{init_database, verificar_conexion, cerrar_pool, obtener_saldo, obtener_total_monedas, obtener_monedas_minadas, obtener_monedas_disponibles};
use crate::logs::log_event;
use crate::utils::{limpiar_pantalla, print_verde, print_rojo, print_amarillo, print_azul, print_blanco, print_cyan, input_filtrado};
use crate::crear_monedas::{generar_monedas, verificar_integridad, TOTAL_MONEDAS};
use crate::minar::minar_automatico;

fn signal_handler() {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        println!("\n");
        print_amarillo("Cerrando conexiones de base de datos...");
        let _ = tokio::runtime::Runtime::new().unwrap().block_on(cerrar_pool()); // cerrar conexiones con la db
        print_verde("Sistema cerrado correctamente");
        r.store(false, Ordering::SeqCst);
        process::exit(0);
    }).expect("Error setting Ctrl-C handler");
}

fn verificar_clave_crypto() -> bool {
    match obtener_clave_crypto() {
        Some(clave) if clave.len() == 32 => true, 
        _ => {
            print_rojo("ERROR: Clave criptografica no valida");
            false // retorna falsa, error
        }
    }
}

async fn verificar_postgresql() -> bool { 
    let (config_valida, errores) = verificar_configuracion_postgres();
    if !config_valida {
        print_rojo("ERROR: Configuracion de PostgreSQL incompleta");
        for error in errores {
            print_rojo(&format!("  - {}", error));
        }
        print_amarillo("Revisa el archivo .env con las credenciales"); // manejo de error
        return false;
    }

    if !verificar_conexion().await {
        print_rojo("ERROR: No se pudo conectar a PostgreSQL");
        print_amarillo("Verifica que PostgreSQL este ejecutandose");
        return false; 
    }

    true 
}

async fn mostrar_saldo() -> i64 { 
    match obtener_saldo().await {
        Ok(saldo) => saldo, 
        Err(_) => 0 
    }
}

async fn mostrar_estado() -> (i64, i64, i64) {
    let total = match obtener_total_monedas().await {
        Ok(t) => t,
        Err(e) => {
            print_rojo(&format!("Error al obtener total monedas: {}", e));
            return (0, 0, 0);
        }
    };

    let minadas = match obtener_monedas_minadas().await {
        Ok(m) => m,
        Err(e) => {
            print_rojo(&format!("Error al obtener monedas minadas: {}", e));
            return (0, 0, 0);
        }
    };

    let disponibles = match obtener_monedas_disponibles().await {
        Ok(d) => d,
        Err(e) => {
            print_rojo(&format!("Error al obtener monedas disponibles: {}", e));
            return (0, 0, 0);
        }
    };

    let saldo = mostrar_saldo().await;
    
    print_amarillo("\n=== ESTADO DEL SISTEMA ===");
    
    if total > 0 {
        let porcentaje = (minadas as f64 / total as f64) * 100.0;
        print_blanco(&format!("Total monedas: {}", total));
        print_blanco(&format!("Monedas minadas: {} ({:.2}%)", minadas, porcentaje));
        print_blanco(&format!("Monedas disponibles: {}", disponibles));
        print_blanco(&format!("Saldo: ${}", saldo));
        print_blanco("Cifrado: AES-256-GCM (individual por moneda)");
        print_blanco("Base de datos: PostgreSQL");
    } else {
        print_rojo("No hay monedas en el sistema");
        print_amarillo(&format!("Ejecuta 'generar' para crear {} monedas", TOTAL_MONEDAS));
    }
    
    (total, minadas, disponibles)
}

fn mostrar_ayuda() {
    print_amarillo("\nCOMANDOS:");
    print_blanco("  generar   - Generar 1,000,000 monedas");
    print_blanco("  minar     - Minar monedas automaticamente");
    print_blanco("  estado    - Ver estado del sistema");
    print_blanco("  saldo     - Ver saldo actual");
    print_blanco("  reiniciar - Reiniciar sistema (elimina TODOS los datos)");
    print_blanco("  verificar - Verificar integridad de las monedas");
    print_blanco("  help      - Esta ayuda");
    print_blanco("  salir     - Salir del programa");
    print_azul("\n  Cifrado: AES-256-GCM individual por moneda");
    print_azul("  Base de datos: PostgreSQL");
    println!();
}

fn confirmar_reinicio() -> bool {
    print_rojo("\nADVERTENCIA: Esto eliminara TODOS los datos del sistema");
    print_rojo("  - 1,000,000 monedas generadas");
    print_rojo("  - Saldo acumulado");
    print_rojo("  - Historial de minado");
    print_rojo("  - Logs del sistema");
    print_verde("  - La clave criptografica se conservara");
    print_rojo("Esta accion NO se puede deshacer");
    println!();

    loop {
        let respuesta = input_filtrado("Confirmar reinicio? (s/n): ");
        match respuesta.to_lowercase().trim() {
            "s" => return true,
            "n" => return false,
            _ => print_rojo("Escribe 's' para si o 'n' para no"),
        }
    }
}

async fn comando_generar() {
    print_amarillo("\n=== GENERACION DE MONEDAS ===");
    print_blanco(&format!("Total a generar: {} monedas", TOTAL_MONEDAS));
    print_azul("Este proceso puede tomar varios minutos");
    println!();

    let respuesta = input_filtrado("Deseas continuar? (s/n): ");
    if respuesta.to_lowercase().trim() != "s" {
        print_azul("Generacion cancelada");
        input_filtrado("\nPresiona ENTER para continuar...");
        return;
    }

    println!();
    if generar_monedas(10000).await {
        print_verde("\nMonedas generadas exitosamente");
    } else {
        print_rojo("\nError al generar monedas");
        print_rojo("Revisa sistema.log para mas informacion");
    }

    input_filtrado("\nPresiona ENTER para continuar...");
}

async fn comando_minar() {
    let total = match obtener_total_monedas().await {
        Ok(t) => t,
        Err(e) => {
            print_rojo(&format!("Error al obtener total monedas: {}", e));
            input_filtrado("\nPresiona ENTER para continuar...");
            return;
        }
    };

    let minadas = match obtener_monedas_minadas().await {
        Ok(m) => m,
        Err(e) => {
            print_rojo(&format!("Error al obtener monedas minadas: {}", e));
            input_filtrado("\nPresiona ENTER para continuar...");
            return;
        }
    };

    let disponibles = match obtener_monedas_disponibles().await {
        Ok(d) => d,
        Err(e) => {
            print_rojo(&format!("Error al obtener monedas disponibles: {}", e));
            input_filtrado("\nPresiona ENTER para continuar...");
            return;
        }
    };

    if total == 0 {
        print_rojo("\nNo hay monedas en el sistema");
        print_amarillo("Ejecuta 'generar' primero para crear las monedas");
        input_filtrado("\nPresiona ENTER para continuar...");
        return;
    }

    if disponibles == 0 {
        print_verde("\nTodas las monedas ya han sido minadas");
        input_filtrado("\nPresiona ENTER para continuar...");
        return;
    }

    let saldo = mostrar_saldo().await;
    print_amarillo("\n=== MINADO DE MONEDAS ===");
    print_azul(&format!(
        "Total: {} monedas | Minadas: {} | Disponibles: {} | Saldo: ${}",
        total, minadas, disponibles, saldo
    ));
    print_azul("Cifrado: AES-256-GCM individual por moneda");
    print_cyan("\nPresiona 'N' en cualquier momento para detener el minado");
    println!();

    minar_automatico().await;
    print_azul("\nMinado detenido. Volviendo al menu principal...");
    input_filtrado("\nPresiona ENTER para continuar...");
}

async fn comando_verificar() {
    print_amarillo("\n=== VERIFICACION DE INTEGRIDAD ===");
    if verificar_integridad().await {
        print_verde("Sistema verificado correctamente");
    } else {
        print_rojo("Se encontraron errores en el sistema");
        print_rojo("Ejecuta 'reiniciar' y luego 'generar' para reconstruir");
    }
    input_filtrado("\nPresiona ENTER para continuar...");
}

async fn inicializar_sistema() {
    let _ = log_event("Sistema iniciado con PostgreSQL");

    if !verificar_clave_crypto() {
        print_rojo("No se puede continuar sin una clave criptografica valida");
        print_amarillo("La clave se genera automaticamente en crypto_key.key");
        input_filtrado("\nPresiona ENTER para salir...");
        process::exit(1);
    }

    if !verificar_postgresql().await {
        print_rojo("No se puede continuar sin una conexion valida a PostgreSQL");
        input_filtrado("\nPresiona ENTER para salir...");
        process::exit(1);
    }

    if !init_database().await {
        print_rojo("Error al inicializar la base de datos");
        input_filtrado("\nPresiona ENTER para salir...");
        process::exit(1);
    }
}

#[tokio::main]
async fn main() {
    signal_handler();
    inicializar_sistema().await;

    loop {
        limpiar_pantalla();
        let comando = input_filtrado(">>> ").to_lowercase();

        match comando.trim() {
            "generar" => {
                limpiar_pantalla();
                comando_generar().await;
            },
            "minar" => {
                limpiar_pantalla();
                comando_minar().await;
            },
            "saldo" => {
                limpiar_pantalla();
                print_amarillo("\n=== SALDO ACTUAL ===");
                let saldo = mostrar_saldo().await;
                print_verde(&format!("${}", saldo));
                input_filtrado("\nPresiona ENTER para continuar...");
            },
            "estado" => {
                limpiar_pantalla();
                mostrar_estado().await;
                input_filtrado("\nPresiona ENTER para continuar...");
            },
            "verificar" => {
                limpiar_pantalla();
                comando_verificar().await;
            },
            "reiniciar" => {
                limpiar_pantalla();
                if confirmar_reinicio() {
                    print_amarillo("Reiniciando sistema...");
                    crate::reiniciar::eliminar_archivo_log();
                    if crate::reiniciar::reiniciar_sistema_postgres(false, true, true).await {
                        print_verde("Sistema reiniciado correctamente");
                        let _ = log_event("Sistema reiniciado por el usuario");
                        print_amarillo("Ejecuta 'generar' para crear nuevas monedas");
                    } else {
                        print_rojo("Error al reiniciar el sistema");
                    }
                    input_filtrado("\nPresiona ENTER para continuar...");
                } else {
                    print_azul("Reinicio cancelado");
                    input_filtrado("\nPresiona ENTER para continuar...");
                }
            },
            "help" | "ayuda" => {
                limpiar_pantalla();
                mostrar_ayuda();
                input_filtrado("Presiona ENTER para continuar...");
            },
            "salir" | "exit" | "quit" => {
                print_azul("\nSaliendo...");
                let _ = log_event("Programa cerrado por el usuario");
                cerrar_pool().await;
                process::exit(0);
            },
            "" => continue,
            _ => {
                print_rojo(&format!("\nComando desconocido: '{}'", comando));
                print_blanco("Escribe 'help' para ver comandos disponibles");
                input_filtrado("\nPresiona ENTER para continuar...");
            }
        }
    }
}