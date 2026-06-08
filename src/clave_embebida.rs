use base64::engine::general_purpose::STANDARD;
use base64::Engine;

const CLAVE_BASE64: &str = "7xKj9PmQ2LrN8vXy5BcF3wAeRtY6uIoP1zC4bVgH7jM9kL0nBvCxZ2aQsW5eR8tY";

pub fn obtener_clave_embebida() -> Vec<u8> {
    match STANDARD.decode(CLAVE_BASE64) {
        Ok(clave) => {
            if clave.len() == 32 {
                clave
            } else {
                generar_clave_fallback()
            }
        }
        Err(_) => generar_clave_fallback()
    }
}

fn generar_clave_fallback() -> Vec<u8> {
    let clave_fija: [u8; 32] = [
        0x5F, 0x8C, 0xA3, 0x1E, 0x4D, 0x7B, 0x92, 0xF6,
        0x2A, 0xC5, 0x8E, 0x17, 0x3B, 0xD9, 0x64, 0x0F,
        0x9E, 0x2C, 0x7A, 0x4B, 0x85, 0x1D, 0xF3, 0xA6,
        0xC8, 0x3F, 0x5E, 0x91, 0x2D, 0x6B, 0x47, 0xE0
    ];
    clave_fija.to_vec()
}