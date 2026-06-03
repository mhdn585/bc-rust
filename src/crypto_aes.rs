use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, KeyInit};
use rand::RngCore;
use rand::rngs::OsRng;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use crate::logs::log_error;

pub fn cifrar_datos_aes(datos_bytes: &[u8], clave: &[u8]) -> Option<(Vec<u8>, Vec<u8>, Vec<u8>)> {
    if clave.len() != 32 {
        log_error(&format!("Clave invalida para cifrado: longitud {}", clave.len()));
        return None;
    }

    let mut nonce_bytes = vec![0u8; 12];
    if OsRng.try_fill_bytes(&mut nonce_bytes).is_err() {
        log_error("Error al generar nonce");
        return None;
    }

    let key = Key::<Aes256Gcm>::from_slice(clave);
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(&nonce_bytes);

    match cipher.encrypt(nonce, datos_bytes) {
        Ok(ciphertext) => {
            let tag = ciphertext[ciphertext.len() - 16..].to_vec();
            let encrypted_data = ciphertext[..ciphertext.len() - 16].to_vec();
            Some((encrypted_data, nonce_bytes, tag))
        }
        Err(e) => {
            log_error(&format!("Error en cifrado AES-256: {}", e));
            None
        }
    }
}

pub fn descifrar_datos_aes(ciphertext: &[u8], nonce: &[u8], tag: &[u8], clave: &[u8]) -> Option<Vec<u8>> {
    if clave.len() != 32 {
        log_error(&format!("Clave invalida para descifrado: longitud {}", clave.len()));
        return None;
    }

    if nonce.len() != 12 {
        log_error(&format!("Nonce invalido: longitud {} (debe ser 12)", nonce.len()));
        return None;
    }

    if tag.len() != 16 {
        log_error(&format!("Tag invalido: longitud {} (debe ser 16)", tag.len()));
        return None;
    }

    let mut ciphertext_con_tag = ciphertext.to_vec();
    ciphertext_con_tag.extend_from_slice(tag);

    let key = Key::<Aes256Gcm>::from_slice(clave);
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(nonce);

    match cipher.decrypt(nonce, ciphertext_con_tag.as_ref()) {
        Ok(plaintext) => Some(plaintext),
        Err(e) => {
            log_error(&format!("Error en descifrado AES-256: {}", e));
            None
        }
    }
}