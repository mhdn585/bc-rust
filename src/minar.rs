use std::io::{self, Write};
use std::time::Duration;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use base64::Engine;
use crossterm::{event::{self, Event, KeyCode}, terminal};
use crate::logs::{log_error, log_event};
use crate::utils::{print_verde, print_rojo, print_amarillo, print_azul, print_blanco, print_cyan};
use crate::config::obtener_clave_crypto;
use crate::crypto_aes::descifrar_datos_aes;
use crate::db::{
    obtener_siguiente_moneda_no_minada, actualizar_estado_moneda, actualizar_saldo,
    obtener_saldo, obtener_total_monedas, obtener_monedas_minadas, obtener_monedas_disponibles,
    verificar_id_original_existe
};
use crate::crear_monedas::VALOR_MERCURY;

const VELOCIDAD_DESCIFRADO: f64 = 0.05;

fn tecla_n_presionada() -> bool {
    if event::poll(Duration::from_millis(0)).unwrap_or(false) {
        if let Ok(Event::Key(key_event)) = event::read() {
            return key_event.code == KeyCode::Char('n') || key_event.code == KeyCode::Char('N');
        }
    }
    false
}

fn descifrar_id_moneda(id_cifrado_b64: &str, clave_aes: &[u8]) -> Option<String> {
    let datos_combinados = match base64::engine::general_purpose::STANDARD.decode(id_cifrado_b64) {
        Ok(d) => d,
        Err(e) => {
            log_error(&format!("Error al decodificar ID cifrado: {}", e));
            return None;
        }
    };

    if datos_combinados.len() < 28 {
        log_error(&format!("Datos demasiado cortos para descifrar: {} bytes", datos_combinados.len()));
        return None;
    }

    let nonce = &datos_combinados[0..12];
    let tag = &datos_combinados[12..28];
    let ciphertext = &datos_combinados[28..];

    let id_descifrado_bytes = match descifrar_datos_aes(ciphertext, nonce, tag, clave_aes) {
        Some(d) => d,
        None => return None,
    };

    match String::from_utf8(id_descifrado_bytes) {
        Ok(s) => Some(s),
        Err(e) => {
            log_error(&format!("Error al convertir ID descifrado a UTF-8: {}", e));
            None
        }
    }
}

fn mostrar_transformacion_descifrado(id_cifrado: &str, id_original: &str, stop_flag: &Arc<AtomicBool>) -> Option<String> {
    let cifrado_len = id_cifrado.len();
    let original_len = id_original.len();

    println!();
    print_amarillo("+------------------------------------------------------------+");
    print_amarillo("|           DESCIFRADO EN VIVO - MERCURY                    |");
    print_amarillo("+------------------------------------------------------------+");
    println!();

    print_blanco("ID CIFRADO (AES-256-GCM):");
    print_azul(id_cifrado);
    println!();
    print_cyan("Descifrando moneda Mercury automaticamente...");
    println!();

    let mut texto_descifrado = String::new();
    for i in 0..original_len {
        if stop_flag.load(Ordering::SeqCst) || tecla_n_presionada() {
            stop_flag.store(true, Ordering::SeqCst);
            println!();
            print_amarillo("\n[MINADO DETENIDO] Usuario solicito detener el proceso");
            return None;
        }

        texto_descifrado.push(id_original.chars().nth(i).unwrap());

        let proporcion = (i + 1) as f64 / original_len as f64;
        let chars_visibles_cifrado = (cifrado_len as f64 * (1.0 - proporcion)) as usize;

        let cifrado_visible = if chars_visibles_cifrado > 0 {
            &id_cifrado[0..chars_visibles_cifrado.min(cifrado_len)]
        } else {
            ""
        };

        print!("\r\x1b[K");
        if !cifrado_visible.is_empty() {
            print!("\x1b[91m{}\x1b[0m -> \x1b[92m{}\x1b[0m", cifrado_visible, texto_descifrado);
        } else {
            print!("\x1b[92m-> {}\x1b[0m", texto_descifrado);
        }
        io::stdout().flush().unwrap();

        std::thread::sleep(Duration::from_secs_f64(VELOCIDAD_DESCIFRADO));
    }

    println!();
    println!();
    print_blanco("ID ORIGINAL COMPLETO:");
    print_verde(&texto_descifrado);
    println!();

    Some(texto_descifrado)
}

fn mostrar_animacion_verificacion(stop_flag: &Arc<AtomicBool>) -> bool {
    let frames = ["-", "\\", "|", "/"];
    for _ in 0..12 {
        if stop_flag.load(Ordering::SeqCst) || tecla_n_presionada() {
            stop_flag.store(true, Ordering::SeqCst);
            return false;
        }
        for frame in frames {
            print!("\r  {} Verificando autenticidad contra base de datos...", frame);
            io::stdout().flush().unwrap();
            std::thread::sleep(Duration::from_millis(50));
            if stop_flag.load(Ordering::SeqCst) || tecla_n_presionada() {
                stop_flag.store(true, Ordering::SeqCst);
                return false;
            }
        }
    }
    println!("\r  [OK] Verificacion completada                              ");
    std::thread::sleep(Duration::from_millis(300));
    true
}

fn mostrar_animacion_minado(stop_flag: &Arc<AtomicBool>) -> bool {
    for i in 1..=20 {
        if stop_flag.load(Ordering::SeqCst) || tecla_n_presionada() {
            stop_flag.store(true, Ordering::SeqCst);
            return false;
        }
        let barra = "#".repeat(i) + &".".repeat(20 - i);
        let porcentaje = i * 5;
        print!("\r  Minando Mercury: [{}] {}%", barra, porcentaje);
        io::stdout().flush().unwrap();
        std::thread::sleep(Duration::from_millis(30));
    }
    println!("\r  [OK] Mercury minado exitosamente                              ");
    true
}

fn esperar_enter_para_comenzar() {
    print_cyan("\nPresiona ENTER para comenzar el minado automatico de Mercury...");
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
}

async fn minar_moneda_individual(
    moneda_id: i32,
    id_cifrado: &str,
    numero_moneda: i32,
    total_disponibles: i64,
    actual: i64,
    clave_aes: &[u8],
    stop_flag: &Arc<AtomicBool>
) -> (bool, String) {
    if stop_flag.load(Ordering::SeqCst) {
        return (false, "Minado detenido por usuario".to_string());
    }

    println!();
    print_amarillo(&format!("+------------------------------------------------------------+"));
    print_amarillo(&format!("| MINANDO MERCURY #{}", numero_moneda));
    print_amarillo(&format!("| Valor: ${:.3} USD", VALOR_MERCURY as f64 / 1000.0));
    print_amarillo(&format!("| Progreso: {}/{}", actual, total_disponibles));
    print_amarillo(&format!("+------------------------------------------------------------+"));
    println!();

    let id_original = match descifrar_id_moneda(id_cifrado, clave_aes) {
        Some(id) => id,
        None => {
            print_rojo("  [ERROR] No se pudo descifrar el ID");
            log_error(&format!("No se pudo descifrar la moneda Mercury #{}", numero_moneda));
            return (false, "Error de descifrado".to_string());
        }
    };

    let id_descifrado = match mostrar_transformacion_descifrado(id_cifrado, &id_original, stop_flag) {
        Some(id) => id,
        None => return (false, "Descifrado interrumpido".to_string()),
    };

    if stop_flag.load(Ordering::SeqCst) {
        return (false, "Minado detenido por usuario".to_string());
    }

    println!();
    let verificacion_exitosa = mostrar_animacion_verificacion(stop_flag);
    if !verificacion_exitosa {
        return (false, "Minado detenido por usuario".to_string());
    }

    if stop_flag.load(Ordering::SeqCst) {
        return (false, "Minado detenido por usuario".to_string());
    }

    let existe = verificar_id_original_existe(&id_descifrado).await;

    if existe {
        print_verde("  [OK] ID VALIDO - La moneda Mercury es autentica");
        println!();
        
        let minado_exitoso = mostrar_animacion_minado(stop_flag);
        if !minado_exitoso {
            return (false, "Minado detenido por usuario".to_string());
        }

        if stop_flag.load(Ordering::SeqCst) {
            return (false, "Minado detenido por usuario".to_string());
        }

        if actualizar_estado_moneda(moneda_id, true).await {
            let preview = if id_descifrado.len() > 100 {
                Some(&id_descifrado[0..100])
            } else {
                Some(id_descifrado.as_str())
            };
            let saldo_nuevo = match actualizar_saldo(VALOR_MERCURY, Some(moneda_id), preview).await {
                Ok(s) => s,
                Err(e) => {
                    log_error(&format!("Error al actualizar saldo: {}", e));
                    0
                }
            };

            println!();
            print_verde(&format!("\n  [OK] MERCURY #{} MINADO EXITOSAMENTE", numero_moneda));
            print_verde(&format!("  Ganaste: ${:.3} USD", VALOR_MERCURY as f64 / 1000.0));
            print_blanco(&format!("  Saldo actual: ${:.3} USD", saldo_nuevo as f64 / 1000.0));
            println!();

            let _ = log_event(&format!("Moneda Mercury #{} minada exitosamente, valor ${}", numero_moneda, VALOR_MERCURY as f64 / 1000.0));
            return (true, "Minada exitosamente".to_string());
        } else {
            print_rojo("  [ERROR] No se pudo actualizar el estado de la moneda");
            return (false, "Error al actualizar estado".to_string());
        }
    } else {
        print_rojo("  [ERROR] ID INVALIDO - La moneda Mercury NO es autentica");
        print_rojo("  El ID descifrado no existe en el sistema");
        log_error(&format!("ID invalido detectado en moneda Mercury #{}", numero_moneda));
        return (false, "ID invalido".to_string());
    }
}

pub async fn minar_automatico() {
    let _ = terminal::disable_raw_mode();
    
    let clave_aes = match obtener_clave_crypto() {
        Some(c) => c,
        None => {
            print_rojo("ERROR: No se pudo obtener la clave AES");
            log_error("No se pudo obtener la clave AES para minado");
            return;
        }
    };

    let total_monedas = match obtener_total_monedas().await {
        Ok(t) => t,
        Err(e) => {
            log_error(&format!("Error al obtener total monedas: {}", e));
            print_rojo(&format!("Error: {}", e));
            return;
        }
    };

    let minadas_antes = match obtener_monedas_minadas().await {
        Ok(m) => m,
        Err(e) => {
            log_error(&format!("Error al obtener monedas minadas: {}", e));
            print_rojo(&format!("Error: {}", e));
            return;
        }
    };

    let disponibles = match obtener_monedas_disponibles().await {
        Ok(d) => d,
        Err(e) => {
            log_error(&format!("Error al obtener monedas disponibles: {}", e));
            print_rojo(&format!("Error: {}", e));
            return;
        }
    };

    if total_monedas == 0 {
        print_rojo("No hay monedas Mercury en el sistema");
        print_amarillo("Ejecuta 'generar' primero para crear las monedas Mercury");
        return;
    }

    if disponibles == 0 {
        print_verde("Todas las monedas Mercury ya han sido minadas");
        print_blanco(&format!("Total: {} monedas Mercury - Todas minadas", total_monedas));
        let valor_total = minadas_antes * VALOR_MERCURY;
        print_blanco(&format!("Valor total minado: ${:.3} USD", valor_total as f64 / 1000.0));
        return;
    }

    let saldo_actual = obtener_saldo().await.unwrap_or(0);

    print_azul("+------------------------------------------------------------+");
    print_azul("|              MINADO AUTOMATICO - MERCURY                  |");
    print_azul("+------------------------------------------------------------+");
    println!();
    print_blanco(&format!("Total monedas Mercury: {}", total_monedas));
    print_blanco(&format!("Monedas minadas: {}", minadas_antes));
    print_blanco(&format!("Monedas disponibles: {}", disponibles));
    print_blanco(&format!("Valor por Mercury: ${:.3} USD", VALOR_MERCURY as f64 / 1000.0));
    print_blanco(&format!("Saldo actual: ${:.3} USD", saldo_actual as f64 / 1000.0));
    print_azul("Cifrado: AES-256-GCM");
    print_cyan("\nPresiona 'N' en cualquier momento para detener el minado");

    esperar_enter_para_comenzar();

    if let Err(e) = terminal::enable_raw_mode() {
        log_error(&format!("Error al habilitar modo raw: {}", e));
    }

    let stop_flag = Arc::new(AtomicBool::new(false));
    let mut monedas_minadas_exitosas = 0;
    let mut monedas_con_error = 0;
    let mut disponibles_restantes = disponibles;
    let mut detenido_por_usuario = false;

    while disponibles_restantes > 0 {
        if stop_flag.load(Ordering::SeqCst) || tecla_n_presionada() {
            stop_flag.store(true, Ordering::SeqCst);
            detenido_por_usuario = true;
            print_amarillo("\n\n[MINADO DETENIDO] Usuario solicito detener el proceso");
            break;
        }

        let monedas_pendientes = obtener_siguiente_moneda_no_minada(1).await;

        if monedas_pendientes.is_empty() {
            print_verde("\n  No hay mas monedas Mercury disponibles para minar");
            break;
        }

        let moneda = &monedas_pendientes[0];
        let numero_moneda = moneda.id;
        let actual = monedas_minadas_exitosas + monedas_con_error + 1;

        let (exito, mensaje) = minar_moneda_individual(
            numero_moneda,
            &moneda.id_cifrado,
            numero_moneda,
            disponibles,
            actual as i64,
            &clave_aes,
            &stop_flag
        ).await;

        if stop_flag.load(Ordering::SeqCst) {
            detenido_por_usuario = true;
            if !mensaje.contains("detenido") {
                print_amarillo("\n\n[MINADO DETENIDO] Usuario solicito detener el proceso");
            }
            break;
        }

        if exito {
            monedas_minadas_exitosas += 1;
        } else {
            monedas_con_error += 1;
            if mensaje.contains("detenido") {
                detenido_por_usuario = true;
                break;
            }
        }

        disponibles_restantes -= 1;
    }

    let _ = terminal::disable_raw_mode();

    println!();
    print_amarillo("+------------------------------------------------------------+");
    print_amarillo("|                     RESUMEN FINAL                          |");
    print_amarillo("+------------------------------------------------------------+");
    println!();
    
    if detenido_por_usuario {
        print_amarillo("*** MINADO DETENIDO POR EL USUARIO ***");
        println!();
    }
    
    print_blanco(&format!("Monedas Mercury procesadas: {}", monedas_minadas_exitosas + monedas_con_error));
    print_verde(&format!("Monedas Mercury minadas exitosamente: {}", monedas_minadas_exitosas));
    if monedas_con_error > 0 {
        print_rojo(&format!("Monedas con error: {}", monedas_con_error));
    }

    let ganancia_total = monedas_minadas_exitosas * VALOR_MERCURY;
    print_verde(&format!("Ganancia total en esta sesion: ${:.3} USD", ganancia_total as f64 / 1000.0));

    let saldo_final = obtener_saldo().await.unwrap_or(0);
    let saldo_inicial_valor = minadas_antes * VALOR_MERCURY;
    print_blanco(&format!("Saldo inicial: ${:.3} USD", saldo_inicial_valor as f64 / 1000.0));
    print_verde(&format!("Saldo final: ${:.3} USD", saldo_final as f64 / 1000.0));
    print_azul(&format!("Incremento: +${:.3} USD", (saldo_final - (minadas_antes * VALOR_MERCURY)) as f64 / 1000.0));

    let total_minadas_final = obtener_monedas_minadas().await.unwrap_or(0);
    print_blanco(&format!("Total monedas Mercury minadas: {}", total_minadas_final));
    let valor_total_minado = total_minadas_final * VALOR_MERCURY;
    print_blanco(&format!("Valor total minado acumulado: ${:.3} USD", valor_total_minado as f64 / 1000.0));

    if total_minadas_final == total_monedas {
        print_verde("\n  *** TODAS LAS MERCURY HAN SIDO MINADAS ***");
        print_verde(&format!("  Valor total generado: ${:.3} USD", (total_monedas * VALOR_MERCURY) as f64 / 1000.0));
    }

    let _ = log_event(&format!("Minado completado: {} monedas Mercury minadas, {} errores, ganancia ${:.3} USD, detenido: {}", monedas_minadas_exitosas, monedas_con_error, ganancia_total as f64 / 1000.0, detenido_por_usuario));
}