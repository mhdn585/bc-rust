use std::io::{self, Write};
use std::time::Duration;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use base64::Engine;
use crossterm::{event::{self, Event, KeyCode}, terminal};
use crate::logs::{log_error, log_event};
use crate::utils::{print_verde, print_rojo, print_amarillo, print_azul, print_blanco, print_cyan};
use crate::config::{obtener_clave_crypto, obtener_tiempo_minado};
use crate::crypto_aes::descifrar_datos_aes;
use crate::db::{
    obtener_siguiente_moneda_no_minada, actualizar_porcentaje_moneda, actualizar_saldo,
    obtener_saldo, obtener_total_monedas, obtener_monedas_minadas_completas, obtener_monedas_disponibles,
    verificar_id_original_existe, obtener_porcentaje_moneda
};
use crate::crear_monedas::{VALOR_MERCURY, LONGITUD_ID};

fn obtener_velocidad_descifrado() -> f64 {
    let tiempo_total_segundos = obtener_tiempo_minado() as f64;
    let pausa_por_caracter = tiempo_total_segundos / LONGITUD_ID as f64;
    let velocidad = pausa_por_caracter.max(0.001).min(10.0);
    
    let _ = log_event(&format!("Velocidad de descifrado configurada: {:.4}s por caracter ({}s por moneda completa)", 
        velocidad, tiempo_total_segundos));
    
    velocidad
}

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

fn mostrar_progreso_simple(porcentaje: f64) {
    print!("\r\x1b[K");
    print!("  ({:.2}%)", porcentaje);
    io::stdout().flush().unwrap();
}

fn descifrar_mostrando_progreso(
    id_original: &str, 
    porcentaje_inicial: f64, 
    stop_flag: &Arc<AtomicBool>
) -> Option<(String, f64)> {
    let velocidad = obtener_velocidad_descifrado();
    let original_len = id_original.len();
    
    let inicio_desde = (porcentaje_inicial / 100.0) * original_len as f64;
    let inicio_idx = inicio_desde.round() as usize;

    let mut texto_descifrado = id_original[0..inicio_idx].to_string();
    let mut nuevo_porcentaje = porcentaje_inicial;

    if porcentaje_inicial > 0.0 {
        mostrar_progreso_simple(porcentaje_inicial);
        std::thread::sleep(Duration::from_millis(500));
    }

    for i in inicio_idx..original_len {
        if stop_flag.load(Ordering::SeqCst) || tecla_n_presionada() {
            stop_flag.store(true, Ordering::SeqCst);
            texto_descifrado.push(id_original.chars().nth(i).unwrap());
            let caracteres_procesados = i + 1;
            nuevo_porcentaje = (caracteres_procesados as f64 / original_len as f64) * 100.0;
            println!();
            print_amarillo(&format!("\n[DESCIFRADO INTERRUMPIDO] Progreso alcanzado: {:.4}%", nuevo_porcentaje));
            return Some((texto_descifrado, nuevo_porcentaje));
        }

        texto_descifrado.push(id_original.chars().nth(i).unwrap());
        let caracteres_procesados = i + 1;
        nuevo_porcentaje = (caracteres_procesados as f64 / original_len as f64) * 100.0;

        mostrar_progreso_simple(nuevo_porcentaje);

        std::thread::sleep(Duration::from_secs_f64(velocidad));
    }

    println!();
    println!();

    Some((texto_descifrado, nuevo_porcentaje))
}

fn esperar_enter_para_comenzar() {
    print_cyan("\nPresiona ENTER para comenzar el minado automatico de Mercury...");
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
}

fn calcular_incremento_porcentaje(porcentaje_previo: f64, porcentaje_nuevo: f64) -> i64 {
    let diferencia = porcentaje_nuevo - porcentaje_previo;
    let valor_obtenido = (diferencia / 100.0) * VALOR_MERCURY as f64;
    valor_obtenido.round() as i64
}

async fn minar_moneda_individual(
    moneda_id: i32,
    id_cifrado: &str,
    numero_moneda: i32,
    clave_aes: &[u8],
    stop_flag: &Arc<AtomicBool>,
    contador_procesadas: &mut i64,
    ganancia_sesion: &mut i64
) -> (bool, String, f64) {
    if stop_flag.load(Ordering::SeqCst) {
        return (false, "Minado detenido por usuario".to_string(), 0.0);
    }

    let porcentaje_actual_db = match obtener_porcentaje_moneda(moneda_id).await {
        Some(p) => p,
        None => {
            print_rojo(&format!("  [ERROR] No se pudo obtener el porcentaje de la moneda #{}", numero_moneda));
            return (false, "Error al obtener porcentaje".to_string(), 0.0);
        }
    };

    let id_original = match descifrar_id_moneda(id_cifrado, clave_aes) {
        Some(id) => id,
        None => {
            print_rojo("  [ERROR] No se pudo descifrar el ID");
            log_error(&format!("No se pudo descifrar la moneda Mercury #{}", numero_moneda));
            return (false, "Error de descifrado".to_string(), porcentaje_actual_db);
        }
    };

    let descifrado_resultado = match descifrar_mostrando_progreso(
        &id_original, 
        porcentaje_actual_db, 
        stop_flag
    ) {
        Some((id, porcentaje)) => (id, porcentaje),
        None => return (false, "Descifrado interrumpido".to_string(), porcentaje_actual_db),
    };
    
    let (id_descifrado, porcentaje_descifrado) = descifrado_resultado;

    if stop_flag.load(Ordering::SeqCst) {
        if porcentaje_descifrado > porcentaje_actual_db + 0.01 {
            let _ = actualizar_porcentaje_moneda(moneda_id, porcentaje_descifrado).await;
            let incremento = calcular_incremento_porcentaje(porcentaje_actual_db, porcentaje_descifrado);
            if incremento > 0 {
                let preview = if id_descifrado.len() > 100 {
                    Some(&id_descifrado[0..100])
                } else {
                    Some(id_descifrado.as_str())
                };
                let _ = actualizar_saldo(incremento, Some(moneda_id), Some(porcentaje_actual_db), Some(porcentaje_descifrado), preview).await;
                *ganancia_sesion += incremento;
                *contador_procesadas += 1;
                println!();
                print_amarillo(&format!("  [GUARDADO] Progreso guardado: {:.4}%", porcentaje_descifrado));
            }
        }
        return (false, "Minado detenido por usuario".to_string(), porcentaje_descifrado);
    }

    println!();

    if stop_flag.load(Ordering::SeqCst) {
        return (false, "Minado detenido por usuario".to_string(), porcentaje_descifrado);
    }

    let existe = verificar_id_original_existe(&id_descifrado).await;

    if existe {
        let mut porcentaje_final = porcentaje_descifrado;
        
        let pasos = 20;
        let incremento_por_paso = (100.0 - porcentaje_descifrado) / pasos as f64;
        
        for paso in 0..=pasos {
            if stop_flag.load(Ordering::SeqCst) || tecla_n_presionada() {
                stop_flag.store(true, Ordering::SeqCst);
                if porcentaje_final > porcentaje_actual_db + 0.01 {
                    let _ = actualizar_porcentaje_moneda(moneda_id, porcentaje_final).await;
                    let incremento = calcular_incremento_porcentaje(porcentaje_actual_db, porcentaje_final);
                    if incremento > 0 {
                        let preview = if id_descifrado.len() > 100 {
                            Some(&id_descifrado[0..100])
                        } else {
                            Some(id_descifrado.as_str())
                        };
                        let _ = actualizar_saldo(incremento, Some(moneda_id), Some(porcentaje_actual_db), Some(porcentaje_final), preview).await;
                        *ganancia_sesion += incremento;
                        *contador_procesadas += 1;
                        println!();
                        print_amarillo(&format!("  [GUARDADO] Progreso guardado: {:.4}%", porcentaje_final));
                    }
                }
                return (false, "Minado detenido por usuario".to_string(), porcentaje_final);
            }
            
            let progreso_actual = porcentaje_descifrado + (paso as f64 * incremento_por_paso);
            porcentaje_final = progreso_actual.min(100.0);
            
            print!("\r\x1b[K");
            print_azul(&format!("  Minando Mercury: ({:.2}%)", porcentaje_final));
            io::stdout().flush().unwrap();
            
            std::thread::sleep(Duration::from_millis(30));
        }
        
        println!();
        print_verde("  [OK] Minado completado!");
        
        let porcentaje_a_guardar = if porcentaje_final >= 99.99 { 100.0 } else { porcentaje_final };
        
        if actualizar_porcentaje_moneda(moneda_id, porcentaje_a_guardar).await {
            let incremento = calcular_incremento_porcentaje(porcentaje_actual_db, porcentaje_a_guardar);
            
            let preview = if id_descifrado.len() > 100 {
                Some(&id_descifrado[0..100])
            } else {
                Some(id_descifrado.as_str())
            };
            
            let _ = match actualizar_saldo(incremento, Some(moneda_id), Some(porcentaje_actual_db), Some(porcentaje_a_guardar), preview).await {
                Ok(s) => s,
                Err(e) => {
                    log_error(&format!("Error al actualizar saldo: {}", e));
                    0
                }
            };

            *ganancia_sesion += incremento;
            *contador_procesadas += 1;

            println!();
            if (porcentaje_a_guardar - 100.0).abs() < 0.0001 {
                print_verde(&format!("\n  [OK] MERCURY #{} MINADO COMPLETAMENTE", numero_moneda));
                print_verde(&format!("  Progreso: 100.00%"));
                print_verde(&format!("  Monedas minadas en esta sesion: {}", contador_procesadas));
                print_verde(&format!("  Ganancia en esta sesion: ${:.3} USD", *ganancia_sesion as f64 / 1000.0));
            } else {
                print_verde(&format!("\n  [OK] MERCURY #{} MINADO PARCIALMENTE", numero_moneda));
                print_verde(&format!("  Progreso: {:.4}%", porcentaje_a_guardar));
                print_verde(&format!("  Monedas minadas en esta sesion: {}", contador_procesadas));
                print_verde(&format!("  Ganancia en esta sesion: ${:.3} USD", *ganancia_sesion as f64 / 1000.0));
            }
            println!();

            let _ = log_event(&format!("Moneda Mercury #{} minada a {:.4}%, incremento ${:.3} USD", 
                numero_moneda, porcentaje_a_guardar, incremento as f64 / 1000.0));
            return (true, "Minada exitosamente".to_string(), porcentaje_a_guardar);
        } else {
            print_rojo("  [ERROR] No se pudo actualizar el porcentaje de la moneda");
            return (false, "Error al actualizar porcentaje".to_string(), porcentaje_descifrado);
        }
    } else {
        print_rojo("  [ERROR] ID INVALIDO - La moneda Mercury NO es autentica");
        print_rojo("  El ID descifrado no existe en el sistema");
        log_error(&format!("ID invalido detectado en moneda Mercury #{}", numero_moneda));
        return (false, "ID invalido".to_string(), porcentaje_descifrado);
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

    let minadas_completas_antes = match obtener_monedas_minadas_completas().await {
        Ok(m) => m,
        Err(e) => {
            log_error(&format!("Error al obtener monedas minadas completas: {}", e));
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

    let disponibles_restantes = disponibles;
    if disponibles_restantes == 0 {
        print_verde("Todas las monedas Mercury ya han sido minadas completamente");
        let valor_total = minadas_completas_antes * VALOR_MERCURY;
        print_blanco(&format!("Valor total minado: ${:.3} USD", valor_total as f64 / 1000.0));
        return;
    }

    let saldo_actual = obtener_saldo().await.unwrap_or(0);
    let tiempo_configurado = obtener_tiempo_minado();

    println!();
    println!("+------------------------------------------------------------+");
    println!("|              MINADO AUTOMATICO - MERCURY                  |");
    println!("+------------------------------------------------------------+");
    println!();
    println!("Total monedas Mercury: {}", total_monedas);
    println!("Monedas minadas completas: {}", minadas_completas_antes);
    println!("Monedas disponibles: {}", disponibles_restantes);
    println!("Valor por Mercury: ${:.3} USD", VALOR_MERCURY as f64 / 1000.0);
    println!("Saldo actual: ${:.3} USD", saldo_actual as f64 / 1000.0);
    println!("Tiempo por moneda completa: {} segundos", tiempo_configurado);
    println!("Cifrado: AES-256-GCM");
    println!();
    println!("Presiona 'N' en cualquier momento para detener el minado");
    println!("El progreso se guarda automaticamente al interrumpir");

    esperar_enter_para_comenzar();

    if let Err(e) = terminal::enable_raw_mode() {
        log_error(&format!("Error al habilitar modo raw: {}", e));
    }

    let stop_flag = Arc::new(AtomicBool::new(false));
    let mut monedas_procesadas = 0;
    let mut monedas_con_error = 0;
    let mut disponibles_restantes_actual = disponibles_restantes;
    let mut detenido_por_usuario = false;
    let mut contador_procesadas_sesion = 0;
    let mut ganancia_sesion = 0;

    while disponibles_restantes_actual > 0 {
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
        let porcentaje_antes = moneda.porcentaje_minado;

        let (exito, mensaje, nuevo_porcentaje) = minar_moneda_individual(
            numero_moneda,
            &moneda.id_cifrado,
            numero_moneda,
            &clave_aes,
            &stop_flag,
            &mut contador_procesadas_sesion,
            &mut ganancia_sesion
        ).await;

        if stop_flag.load(Ordering::SeqCst) {
            detenido_por_usuario = true;
            if !mensaje.contains("detenido") {
                print_amarillo("\n\n[MINADO DETENIDO] Usuario solicito detener el proceso");
            }
            break;
        }

        if exito {
            monedas_procesadas += 1;
            if (nuevo_porcentaje - 100.0).abs() < 0.0001 {
                disponibles_restantes_actual -= 1;
            }
        } else {
            monedas_con_error += 1;
            if mensaje.contains("detenido") {
                detenido_por_usuario = true;
                break;
            }
            if porcentaje_antes < 99.99 && nuevo_porcentaje > porcentaje_antes + 0.01 {
                disponibles_restantes_actual -= 1;
            }
        }
    }

    let _ = terminal::disable_raw_mode();

    println!();
    println!("+------------------------------------------------------------+");
    println!("|                     RESUMEN FINAL                          |");
    println!("+------------------------------------------------------------+");
    println!();
    
    if detenido_por_usuario {
        println!("*** MINADO DETENIDO POR EL USUARIO ***");
        println!();
    }
    
    println!("Monedas Mercury procesadas: {}", contador_procesadas_sesion);
    println!("Monedas Mercury completadas: {}", monedas_procesadas);
    if monedas_con_error > 0 {
        println!("Monedas con error: {}", monedas_con_error);
    }

    println!("Ganancia total en esta sesion: ${:.3} USD", ganancia_sesion as f64 / 1000.0);

    let saldo_final = obtener_saldo().await.unwrap_or(0);
    let minadas_completas_final = obtener_monedas_minadas_completas().await.unwrap_or(0);
    let valor_minado_completo = minadas_completas_final * VALOR_MERCURY;
    
    println!("Saldo inicial: ${:.3} USD", saldo_actual as f64 / 1000.0);
    println!("Saldo final: ${:.3} USD", saldo_final as f64 / 1000.0);
    println!("Monedas completadas totales: {}", minadas_completas_final);
    println!("Valor total minado acumulado: ${:.3} USD", valor_minado_completo as f64 / 1000.0);

    if minadas_completas_final == total_monedas {
        println!();
        println!("  *** TODAS LAS MERCURY HAN SIDO MINADAS COMPLETAMENTE ***");
        println!("  Valor total generado: ${:.3} USD", (total_monedas * VALOR_MERCURY) as f64 / 1000.0);
    } else {
        let porcentaje_total = (minadas_completas_final as f64 / total_monedas as f64) * 100.0;
        println!("  Progreso total del sistema: {:.2}% completado", porcentaje_total);
    }

    let _ = log_event(&format!("Minado completado: {} monedas Mercury procesadas, {} completadas, {} errores, ganancia ${:.3} USD, detenido: {}", 
        contador_procesadas_sesion, monedas_procesadas, monedas_con_error, ganancia_sesion as f64 / 1000.0, detenido_por_usuario));
}