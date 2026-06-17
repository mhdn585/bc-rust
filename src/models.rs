use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct MonedaPendiente {
    pub id: i32,
    pub id_cifrado: String,
    pub porcentaje_minado: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Estadisticas {
    pub total_ids_originales: i64,
    pub total_monedas_cifradas: i64,
    pub monedas_minadas_completas: i64,
    pub monedas_minadas_parciales: i64,
    pub monedas_no_minadas: i64,
    pub saldo_actual: i64,
    pub valor_por_moneda: i64,
}

#[allow(dead_code)]
impl Estadisticas {
    pub fn valor_total_sistema_usd(&self) -> f64 {
        (self.total_monedas_cifradas * self.valor_por_moneda) as f64 / 1000.0
    }
    
    pub fn valor_minado_completo_usd(&self) -> f64 {
        (self.monedas_minadas_completas * self.valor_por_moneda) as f64 / 1000.0
    }
    
    pub fn porcentaje_minado_promedio(&self) -> f64 {
        if self.total_monedas_cifradas > 0 {
            let suma_porcentajes = (self.monedas_minadas_completas * 100) + (self.monedas_minadas_parciales * 50);
            suma_porcentajes as f64 / self.total_monedas_cifradas as f64
        } else {
            0.0
        }
    }
    
    pub fn saldo_actual_usd(&self) -> f64 {
        self.saldo_actual as f64 / 1000.0
    }
    
    pub fn porcentaje_minado_completo(&self) -> f64 {
        if self.total_monedas_cifradas > 0 {
            (self.monedas_minadas_completas as f64 / self.total_monedas_cifradas as f64) * 100.0
        } else {
            0.0
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct HistorialTransaccion {
    pub fecha: String,
    pub id_moneda: i32,
    pub incremento: i64,
    pub porcentaje_minado_previo: f64,
    pub porcentaje_minado_nuevo: f64,
    pub valor_usd: String,
    pub saldo_nuevo: i64,
    pub saldo_nuevo_usd: String,
    pub tipo: String,
    pub moneda: String,
    pub id_original_preview: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct SaldoResponse {
    pub saldo: i64,
    pub saldo_usd: f64,
    pub monedas_minadas_completas: i64,
    pub monedas_minadas_parciales: i64,
    pub ultima_actualizacion: String,
}