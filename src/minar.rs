use std::io::{self, Write};
use std::time::Duration;
use base64::Engine;
use crate::logs::{log_error, log_event};
use crate::utils::{print_verde, print_rojo, print_amarillo, print_azul, print_blanco, print_cyan, input_filtrado, get_color_mode};
use crate::config::obtener_clave_crypto;
use crate::crypto_aes::descifrar_datos_aes;
use crate::db::{
    obtener_siguiente_moneda_no_minada, actualizar_estado_moneda, actualizar_saldo,
    obtener_saldo, obtener_total_monedas, obtener_monedas_minadas, obtener_monedas_disponibles,
    verificar_id_original_existe
};

const VELOCIDAD_DESCIFRADO: f64 = 0.05;

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

fn mostrar_transformacion_descifrado(id_cifrado: &str, id_original: &str) -> Option<String> {
    let cifrado_len = id_cifrado.len();
    let original_len = id_original.len();
    let modo_color = get_color_mode();

    println!();
    print_amarillo("+------------------------------------------------------------+");
    print_amarillo("|           DESCIFRADO EN VIVO                               |");
    print_amarillo("+------------------------------------------------------------+");
    println!();

    print_blanco("ID CIFRADO (AES-256-GCM):");
    print_azul(id_cifrado);
    println!();
    print_cyan("Presiona ENTER para iniciar el descifrado...");
    input_filtrado("");

    println!();
    print_blanco("DESCIFRANDO EN VIVO:");
    println!();

    let mut texto_descifrado = String::new();
    for i in 0..original_len {
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
            if modo_color {
                print!("\x1b[91m{}\x1b[0m -> \x1b[92m{}\x1b[0m", cifrado_visible, texto_descifrado);
            } else {
                print!("{} -> {}", cifrado_visible, texto_descifrado);
            }
        } else {
            if modo_color {
                print!("\x1b[92m-> {}\x1b[0m", texto_descifrado);
            } else {
                print!("-> {}", texto_descifrado);
            }
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

fn mostrar_animacion_verificacion() {
    let frames = ["-", "\\", "|", "/"];
    for _ in 0..12 {
        for frame in frames {
            print!("\r  {} Verificando autenticidad contra base de datos...", frame);
            io::stdout().flush().unwrap();
            std::thread::sleep(Duration::from_millis(50));
        }
    }
    println!("\r  [OK] Verificacion completada                              ");
    std::thread::sleep(Duration::from_millis(300));
}

fn mostrar_animacion_minado() {
    for i in 1..=20 {
        let barra = "#".repeat(i) + &".".repeat(20 - i);
        let porcentaje = i * 5;
        print!("\r  Minando: [{}] {}%", barra, porcentaje);
        io::stdout().flush().unwrap();
        std::thread::sleep(Duration::from_millis(30));
    }
    println!("\r  [OK] Moneda minada exitosamente                              ");
}

async fn minar_moneda_individual(
    moneda_id: i32,
    id_cifrado: &str,
    numero_moneda: i32,
    total_disponibles: i64,
    actual: i64,
    clave_aes: &[u8]
) -> (bool, String) {
    println!();
    print_amarillo(&format!("+------------------------------------------------------------+"));
    print_amarillo(&format!("| MINANDO MONEDA #{}", numero_moneda));
    print_amarillo(&format!("| Progreso: {}/{}", actual, total_disponibles));
    print_amarillo(&format!("+------------------------------------------------------------+"));
    println!();

    let id_original = match descifrar_id_moneda(id_cifrado, clave_aes) {
        Some(id) => id,
        None => {
            print_rojo("  [ERROR] No se pudo descifrar el ID");
            log_error(&format!("No se pudo descifrar la moneda #{}", numero_moneda));
            return (false, "Error de descifrado".to_string());
        }
    };

    let id_descifrado = match mostrar_transformacion_descifrado(id_cifrado, &id_original) {
        Some(id) => id,
        None => return (false, "Descifrado interrumpido".to_string()),
    };

    println!();
    mostrar_animacion_verificacion();

    let existe = verificar_id_original_existe(&id_descifrado).await;

    if existe {
        print_verde("  [OK] ID VALIDO - La moneda es autentica");
        println!();
        mostrar_animacion_minado();

        if actualizar_estado_moneda(moneda_id, true).await {
            let preview = if id_descifrado.len() > 100 {
                Some(&id_descifrado[0..100])
            } else {
                Some(id_descifrado.as_str())
            };
            let saldo_nuevo = match actualizar_saldo(1, Some(moneda_id), preview).await {
                Ok(s) => s,
                Err(e) => {
                    log_error(&format!("Error al actualizar saldo: {}", e));
                    0
                }
            };

            println!();
            print_verde(&format!("\n  [OK] MONEDA #{} MINADA EXITOSAMENTE", numero_moneda));
            print_blanco(&format!("  Saldo actual: ${}", saldo_nuevo));
            println!();

            let _ = log_event(&format!("Moneda #{} minada exitosamente", numero_moneda));
            return (true, "Minada exitosamente".to_string());
        } else {
            print_rojo("  [ERROR] No se pudo actualizar el estado de la moneda");
            return (false, "Error al actualizar estado".to_string());
        }
    } else {
        print_rojo("  [ERROR] ID INVALIDO - La moneda NO es autentica");
        print_rojo("  El ID descifrado no existe en el sistema");
        log_error(&format!("ID invalido detectado en moneda #{}", numero_moneda));
        return (false, "ID invalido".to_string());
    }
}

async fn mostrar_mensaje_intermitente() {
    print_cyan("\nPresiona ENTER para comenzar a minar...");
    print!("\n>>> ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
}

pub async fn minar_automatico() {
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
        print_rojo("No hay monedas en el sistema");
        print_amarillo("Ejecuta 'generar' primero para crear las monedas");
        input_filtrado("\nPresiona ENTER para continuar...");
        return;
    }

    if disponibles == 0 {
        print_verde("Todas las monedas ya han sido minadas");
        print_blanco(&format!("Total: {} monedas - Todas minadas", total_monedas));
        input_filtrado("\nPresiona ENTER para continuar...");
        return;
    }

    let saldo_actual = obtener_saldo().await.unwrap_or(0);

    print_azul("+------------------------------------------------------------+");
    print_azul("|                    MINADO AUTOMATICO                       |");
    print_azul("+------------------------------------------------------------+");
    println!();
    print_blanco(&format!("Total monedas: {}", total_monedas));
    print_blanco(&format!("Monedas minadas: {}", minadas_antes));
    print_blanco(&format!("Monedas disponibles: {}", disponibles));
    print_blanco(&format!("Saldo actual: ${}", saldo_actual));
    print_azul("Cifrado: AES-256-GCM");
    print_cyan("\nPresiona 'N' en cualquier momento para detener el minado");
    println!();

    mostrar_mensaje_intermitente().await;

    let mut monedas_minadas_exitosas = 0;
    let mut monedas_con_error = 0;
    let mut disponibles_restantes = disponibles;

    while disponibles_restantes > 0 {
        let monedas_pendientes = obtener_siguiente_moneda_no_minada(1).await;

        if monedas_pendientes.is_empty() {
            print_verde("\n  No hay mas monedas disponibles para minar");
            break;
        }

        let moneda = &monedas_pendientes[0];
        let numero_moneda = moneda.id;
        let actual = monedas_minadas_exitosas + monedas_con_error + 1;

        let (exito, _mensaje) = minar_moneda_individual(
            numero_moneda,
            &moneda.id_cifrado,
            numero_moneda,
            disponibles,
            actual as i64,
            &clave_aes
        ).await;

        if exito {
            monedas_minadas_exitosas += 1;
        } else {
            monedas_con_error += 1;
        }

        disponibles_restantes -= 1;
    }

    println!();
    print_amarillo("+------------------------------------------------------------+");
    print_amarillo("|                     RESUMEN FINAL                          |");
    print_amarillo("+------------------------------------------------------------+");
    println!();
    print_blanco(&format!("Monedas procesadas: {}", monedas_minadas_exitosas + monedas_con_error));
    print_verde(&format!("Monedas minadas exitosamente: {}", monedas_minadas_exitosas));
    if monedas_con_error > 0 {
        print_rojo(&format!("Monedas con error: {}", monedas_con_error));
    }

    let saldo_final = obtener_saldo().await.unwrap_or(0);
    print_blanco(&format!("Saldo inicial: ${}", minadas_antes));
    print_verde(&format!("Saldo final: ${}", saldo_final));
    print_azul(&format!("Incremento: +${}", saldo_final - minadas_antes));

    let total_minadas_final = obtener_monedas_minadas().await.unwrap_or(0);
    print_blanco(&format!("Total monedas minadas: {}", total_minadas_final));

    if total_minadas_final == total_monedas {
        print_verde("\n  *** TODAS LAS MONEDAS HAN SIDO MINADAS ***");
    }

    let _ = log_event(&format!("Minado completado: {} monedas minadas, {} errores", monedas_minadas_exitosas, monedas_con_error));

    println!();
    input_filtrado("Presiona ENTER para continuar...");
}