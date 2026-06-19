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
mod clave_embebida;

use crate::config::{verificar_configuracion_postgres, inicializar_clave_sistema, TOTAL_MONEDAS};
use crate::db::{init_database, verificar_conexion, cerrar_pool, obtener_saldo, obtener_total_monedas, obtener_monedas_minadas_completas, obtener_monedas_disponibles};
use crate::logs::log_event;
use crate::utils::{limpiar_pantalla, print_verde, print_rojo, print_amarillo, print_azul, print_blanco, print_cyan, input_filtrado};
use crate::crear_monedas::{generar_monedas, verificar_integridad, VALOR_MERCURY};
use crate::minar::minar_automatico;

fn signal_handler() {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        println!("\n");
        print_amarillo("Cerrando conexiones de base de datos...");
        let _ = tokio::runtime::Runtime::new().unwrap().block_on(cerrar_pool());
        print_verde("Sistema cerrado correctamente");
        r.store(false, Ordering::SeqCst);
        process::exit(0);
    }).expect("Error setting Ctrl-C handler");
}

fn verificar_clave_sistema() -> bool {
    match inicializar_clave_sistema() {
        Ok(()) => {
            print_verde("Clave criptografica inicializada correctamente");
            true
        }
        Err(e) => {
            print_rojo(&format!("ERROR: No se pudo inicializar la clave criptografica: {}", e));
            false
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
        print_amarillo("Revisa el archivo .env con las credenciales");
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

async fn mostrar_estado() -> (i64, i64, i64, i64) {
    let total = match obtener_total_monedas().await {
        Ok(t) => t,
        Err(e) => {
            print_rojo(&format!("Error al obtener total monedas: {}", e));
            return (0, 0, 0, 0);
        }
    };

    let minadas_completas = match obtener_monedas_minadas_completas().await {
        Ok(m) => m,
        Err(e) => {
            print_rojo(&format!("Error al obtener monedas minadas completas: {}", e));
            return (0, 0, 0, 0);
        }
    };

    let disponibles = match obtener_monedas_disponibles().await {
        Ok(d) => d,
        Err(e) => {
            print_rojo(&format!("Error al obtener monedas disponibles: {}", e));
            return (0, 0, 0, 0);
        }
    };

    let minadas_parciales = total - minadas_completas - disponibles;
    let saldo = mostrar_saldo().await;

    print_amarillo("\n=== ESTADO DEL SISTEMA MERCURY ===");

    if total > 0 {
        let porcentaje_completo = (minadas_completas as f64 / total as f64) * 100.0;
        let porcentaje_parcial = (minadas_parciales as f64 / total as f64) * 100.0;
        let porcentaje_disponible = (disponibles as f64 / total as f64) * 100.0;
        
        print_blanco(&format!("Total monedas Mercury: {}", total));
        print_blanco(&format!("Valor total del sistema: ${:.3} USD", (total * VALOR_MERCURY) as f64 / 1000.0));
        print_verde(&format!("Monedas minadas completas: {} ({:.2}%)", minadas_completas, porcentaje_completo));
        print_azul(&format!("Valor minado completo: ${:.3} USD", (minadas_completas * VALOR_MERCURY) as f64 / 1000.0));
        print_amarillo(&format!("Monedas minadas parciales: {} ({:.2}%)", minadas_parciales, porcentaje_parcial));
        print_blanco(&format!("Monedas disponibles: {} ({:.2}%)", disponibles, porcentaje_disponible));
        print_blanco(&format!("Valor disponible: ${:.3} USD", (disponibles * VALOR_MERCURY) as f64 / 1000.0));
        print_verde(&format!("Saldo acumulado: ${:.3} USD", saldo as f64 / 1000.0));
        print_blanco(&format!("Valor por Mercury: ${:.3} USD", VALOR_MERCURY as f64 / 1000.0));
        print_blanco("Cifrado: AES-256-GCM (individual por moneda)");
        print_blanco("Minado: Fraccionado con porcentaje exacto");
        print_blanco("Base de datos: PostgreSQL");
        print_blanco("Clave: Embebida en el codigo fuente");
    } else {
        print_rojo("No hay monedas Mercury en el sistema");
        print_amarillo(&format!("Ejecuta 'generar' para crear {} monedas Mercury", TOTAL_MONEDAS));
    }

    (total, minadas_completas, minadas_parciales, disponibles)
}

fn mostrar_ayuda() {
    print_amarillo("\nCOMANDOS MERCURY:");
    print_blanco("  generar   - Generar 1,000,000 monedas Mercury (valor $67.998 c/u)");
    print_blanco("  minar     - Minar monedas Mercury (soporta fracciones)");
    print_blanco("  estado    - Ver estado del sistema Mercury");
    print_blanco("  saldo     - Ver saldo actual en USD");
    print_blanco("  reiniciar - Reiniciar sistema (elimina TODOS los datos)");
    print_blanco("  verificar - Verificar integridad de las monedas Mercury");
    print_blanco("  help      - Esta ayuda");
    print_blanco("  salir     - Salir del programa");
    print_azul("\n  Moneda: Mercury - Valor: $67.998 USD cada una");
    print_azul("  Minado fraccionado: Puedes obtener porcentajes exactos");
    print_azul("  Cifrado: AES-256-GCM individual por moneda");
    print_azul("  Base de datos: PostgreSQL");
    print_azul("  Clave: Embebida permanentemente en el sistema");
    println!();
}

fn confirmar_reinicio() -> bool {
    print_rojo("\nADVERTENCIA: Esto eliminara TODOS los datos del sistema");
    print_rojo(&format!("  - {} monedas Mercury generadas", TOTAL_MONEDAS));
    print_rojo(&format!("  - Valor total: ${:.3} USD", (TOTAL_MONEDAS * VALOR_MERCURY) as f64 / 1000.0));
    print_rojo("  - Saldo acumulado");
    print_rojo("  - Historial de minado");
    print_rojo("  - Logs del sistema");
    print_verde("  - La clave criptografica se conserva embebida");
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
    print_amarillo("\n=== GENERACION DE MERCURY ===");
    print_blanco(&format!("Total a generar: {} monedas Mercury", TOTAL_MONEDAS));
    print_blanco(&format!("Valor por moneda: ${:.3} USD", VALOR_MERCURY as f64 / 1000.0));
    print_blanco(&format!("Valor total del sistema: ${:.3} USD", (TOTAL_MONEDAS * VALOR_MERCURY) as f64 / 1000.0));
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
        print_verde("\nMonedas Mercury generadas exitosamente");
    } else {
        print_rojo("\nError al generar monedas Mercury");
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

    let minadas_completas = match obtener_monedas_minadas_completas().await {
        Ok(m) => m,
        Err(e) => {
            print_rojo(&format!("Error al obtener monedas minadas completas: {}", e));
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
        print_rojo("\nNo hay monedas Mercury en el sistema");
        print_amarillo("Ejecuta 'generar' primero para crear las monedas Mercury");
        input_filtrado("\nPresiona ENTER para continuar...");
        return;
    }

    if disponibles == 0 && minadas_completas == total {
        print_verde("\nTodas las monedas Mercury ya han sido minadas completamente");
        let valor_total = minadas_completas * VALOR_MERCURY;
        print_blanco(&format!("Valor total minado: ${:.3} USD", valor_total as f64 / 1000.0));
        input_filtrado("\nPresiona ENTER para continuar...");
        return;
    }

    let saldo = mostrar_saldo().await;
    let minadas_parciales = total - minadas_completas - disponibles;
    
    print_amarillo("\n=== MINADO DE MERCURY ===");
    print_azul(&format!(
        "Total: {} monedas | Completas: {} | Parciales: {} | Disponibles: {} | Saldo: ${:.3} USD",
        total, minadas_completas, minadas_parciales, disponibles, saldo as f64 / 1000.0
    ));
    print_azul(&format!("Valor por Mercury: ${:.3} USD", VALOR_MERCURY as f64 / 1000.0));
    print_azul("Cifrado: AES-256-GCM individual por moneda");
    print_cyan("\nEl minado es fraccionado: puedes obtener porcentajes exactos");
    print_cyan("Presiona 'N' en cualquier momento para detener el minado");
    print_cyan("El progreso se guarda automaticamente");
    println!();

    minar_automatico().await;
    print_azul("\nMinado detenido. Volviendo al menu principal...");
    input_filtrado("\nPresiona ENTER para continuar...");
}

async fn comando_verificar() {
    print_amarillo("\n=== VERIFICACION DE INTEGRIDAD MERCURY ===");
    if verificar_integridad().await {
        print_verde("Sistema Mercury verificado correctamente");
    } else {
        print_rojo("Se encontraron errores en el sistema Mercury");
        print_rojo("Ejecuta 'reiniciar' y luego 'generar' para reconstruir");
    }
    input_filtrado("\nPresiona ENTER para continuar...");
}

async fn inicializar_sistema() {
    let _ = log_event("Sistema Mercury iniciado con PostgreSQL y clave embebida");

    if !verificar_clave_sistema() {
        print_rojo("No se puede continuar sin una clave criptografica valida");
        print_amarillo("La clave esta embebida en el sistema permanentemente");
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
                print_amarillo("\n=== SALDO ACTUAL EN USD ===");
                let saldo = mostrar_saldo().await;
                let monedas_minadas_completas = saldo / VALOR_MERCURY;
                let resto = saldo % VALOR_MERCURY;
                let porcentaje_extra = (resto as f64 / VALOR_MERCURY as f64) * 100.0;
                
                print_verde(&format!("${:.3} USD", saldo as f64 / 1000.0));
                print_blanco(&format!("Equivalente a {} monedas completas", monedas_minadas_completas));
                if resto > 0 {
                    print_blanco(&format!("Mas un {:.2}% de otra moneda", porcentaje_extra));
                }
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
                    print_amarillo("Reiniciando sistema Mercury...");
                    crate::reiniciar::eliminar_archivo_log();
                    if crate::reiniciar::reiniciar_sistema_postgres(false, true, true).await {
                        print_verde("Sistema Mercury reiniciado correctamente");
                        let _ = log_event("Sistema Mercury reiniciado por el usuario");
                        print_amarillo("Ejecuta 'generar' para crear nuevas monedas Mercury");
                    } else {
                        print_rojo("Error al reiniciar el sistema Mercury");
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
                print_azul("\nSaliendo del sistema Mercury...");
                let _ = log_event("Programa Mercury cerrado por el usuario");
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