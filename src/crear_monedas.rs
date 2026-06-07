use rand::Rng;
use base64::Engine;
use crate::logs::log_event;
use crate::utils::{print_verde, print_rojo, print_amarillo, print_blanco, print_azul};
use crate::config::obtener_clave_crypto;
use crate::crypto_aes::cifrar_datos_aes;
use crate::db::{
    insertar_id_original, insertar_moneda_cifrada, verificar_id_original_existe,
    obtener_total_monedas, obtener_estadisticas_completas,
    init_database
};
use std::time::Instant;

pub const TOTAL_MONEDAS: i64 = 1000;
pub const LONGITUD_ID: usize = 1024;


const CARACTERES_PERMITIDOS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()_+-=[]{}|;:,.<>?";


fn generar_id_complejo() -> String {
    let mut rng = rand::thread_rng();
    (0..LONGITUD_ID)
        .map(|_| {
            let idx = rng.gen_range(0..CARACTERES_PERMITIDOS.len());
            CARACTERES_PERMITIDOS[idx] as char
        })
        .collect()
}

async fn generar_id_unico_postgres(intentos_maximos: usize) -> Result<String, String> {
    for intento in 0..intentos_maximos {
        let id_generado = generar_id_complejo();
        let existe = verificar_id_original_existe(&id_generado).await;
        if !existe {
            return Ok(id_generado);
        }
        let _ = log_event(&format!("Colision de ID detectada, reintentando {}/{}", intento + 1, intentos_maximos));
    }
    Err("No se pudo generar un ID unico despues de multiples intentos".to_string())
}

fn cifrar_id_individual(id_original: &str, clave_aes: &[u8]) -> Option<String> {
    let id_bytes = id_original.as_bytes();
    let (ciphertext, nonce, tag) = cifrar_datos_aes(id_bytes, clave_aes)?;
    let mut datos_combinados = nonce;
    datos_combinados.extend_from_slice(&tag);
    datos_combinados.extend_from_slice(&ciphertext);
    Some(base64::engine::general_purpose::STANDARD.encode(&datos_combinados))
}

pub async fn generar_monedas(lote: i64) -> bool {
    let total_existentes = obtener_total_monedas().await.unwrap_or(0);

    if total_existentes >= TOTAL_MONEDAS {
        print_verde(&format!("Ya existen {} monedas en el sistema", total_existentes));
        return true;
    }

    if total_existentes > 0 {
        print_amarillo(&format!("Se encontraron {} monedas existentes", total_existentes));
        print_amarillo(&format!("Faltan generar {} monedas", TOTAL_MONEDAS - total_existentes));
        print!("Deseas continuar con la generacion? (s/n): ");
        let mut respuesta = String::new();
        std::io::stdin().read_line(&mut respuesta).unwrap();
        if respuesta.trim().to_lowercase() != "s" {
            print_azul("Generacion cancelada por el usuario");
            return false;
        }
    }

    let clave_aes = obtener_clave_crypto();
    if clave_aes.is_none() {
        print_rojo("No se pudo obtener la clave AES para cifrar");
        return false;
    }
    let clave_aes = clave_aes.unwrap();

    print_amarillo(&format!("Iniciando generacion de {} monedas", TOTAL_MONEDAS));
    print_azul(&format!("Longitud de IDs: {} caracteres", LONGITUD_ID));
    print_azul("Cifrado: AES-256-GCM individual por moneda");
    print_azul(&format!("Caracteres permitidos: {} tipos", CARACTERES_PERMITIDOS.len()));
    println!();

    if total_existentes == 0 {
        print_blanco("Paso 1/3: Inicializando base de datos...");
        if !init_database().await {
            print_rojo("Error al inicializar la base de datos");
            return false;
        }
        print_verde("Base de datos inicializada correctamente");
        println!();
    }

    let inicio_total = Instant::now();
    let mut monedas_generadas = total_existentes;

    while monedas_generadas < TOTAL_MONEDAS {
        let lote_actual = std::cmp::min(lote, TOTAL_MONEDAS - monedas_generadas);
        let inicio_lote = Instant::now();

        print_blanco(&format!("Generando lote {} - {} de {}", monedas_generadas + 1, monedas_generadas + lote_actual, TOTAL_MONEDAS));

        let mut ids_cifrados_lote = Vec::new();
        let mut ids_originales_lote = Vec::new();

        for i in 0..lote_actual {
            if (i + 1) % 1000 == 0 {
                let porcentaje = ((monedas_generadas + i + 1) as f64 / TOTAL_MONEDAS as f64) * 100.0;
                print_blanco(&format!("  Progreso: {}/{} ({:.2}%)", monedas_generadas + i + 1, TOTAL_MONEDAS, porcentaje));
            }

            let id_original = match generar_id_unico_postgres(5).await {
                Ok(id) => id,
                Err(e) => {
                    let _ = log_event(&format!("Error al generar ID unico: {}", e));
                    return false;
                }
            };

            let id_cifrado = match cifrar_id_individual(&id_original, &clave_aes) {
                Some(id) => id,
                None => {
                    let _ = log_event("Fallo en cifrado, abortando lote");
                    return false;
                }
            };

            ids_originales_lote.push(id_original);
            ids_cifrados_lote.push(id_cifrado);
        }

        for i in 0..lote_actual as usize {
            let _ = insertar_id_original(&ids_originales_lote[i]).await;
            let _ = insertar_moneda_cifrada(&ids_cifrados_lote[i], false).await;
        }

        monedas_generadas += lote_actual;
        let tiempo_lote = inicio_lote.elapsed().as_secs_f64();
        let velocidad = lote_actual as f64 / tiempo_lote;
        let estimado_restante = ((TOTAL_MONEDAS - monedas_generadas) as f64 / velocidad) / 60.0;

        print_verde(&format!("  Lote completado en {:.2}s - Velocidad: {:.0} monedas/seg", tiempo_lote, velocidad));
        print_blanco(&format!("  Tiempo estimado restante: {:.1} minutos", estimado_restante));
        println!();
    }

    let tiempo_total = inicio_total.elapsed().as_secs_f64();
    let minutos_total = tiempo_total / 60.0;

    println!();
    print_verde("=".repeat(60).as_str());
    print_verde("GENERACION COMPLETADA EXITOSAMENTE");
    print_verde("=".repeat(60).as_str());
    print_blanco(&format!("Total de monedas generadas: {}", TOTAL_MONEDAS));
    print_blanco(&format!("Longitud de IDs: {} caracteres", LONGITUD_ID));
    print_blanco("Cifrado: AES-256-GCM (individual por moneda)");
    print_blanco(&format!("Tiempo total: {:.2} segundos ({:.1} minutos)", tiempo_total, minutos_total));
    print_blanco(&format!("Velocidad promedio: {:.0} monedas/seg", TOTAL_MONEDAS as f64 / tiempo_total));
    print_verde("=".repeat(60).as_str());

    let _ = log_event(&format!("Generacion de {} monedas completada en {:.2}s", TOTAL_MONEDAS, tiempo_total));
    true
}

pub async fn verificar_integridad() -> bool {
    let estadisticas = obtener_estadisticas_completas().await;

    print_amarillo("Verificando integridad del sistema...");
    print_blanco(&format!("  IDs originales: {}", estadisticas.total_ids_originales));
    print_blanco(&format!("  Monedas cifradas: {}", estadisticas.total_monedas_cifradas));
    print_blanco(&format!("  Monedas minadas: {}", estadisticas.monedas_minadas));
    print_blanco(&format!("  Monedas disponibles: {}", estadisticas.monedas_disponibles));
    print_blanco(&format!("  Saldo actual: ${}", estadisticas.saldo_actual));

    if estadisticas.total_ids_originales != TOTAL_MONEDAS {
        print_rojo(&format!("ERROR: IDs originales {} != {}", estadisticas.total_ids_originales, TOTAL_MONEDAS));
        return false;
    }

    if estadisticas.total_monedas_cifradas != TOTAL_MONEDAS {
        print_rojo(&format!("ERROR: Monedas cifradas {} != {}", estadisticas.total_monedas_cifradas, TOTAL_MONEDAS));
        return false;
    }

    if estadisticas.monedas_disponibles + estadisticas.monedas_minadas != TOTAL_MONEDAS {
        print_rojo("ERROR: Suma de disponibles + minadas no coincide con total");
        return false;
    }

    print_verde("Verificacion de integridad completada exitosamente");
    true
}