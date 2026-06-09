use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct MonedaPendiente {
    pub id: i32,
    pub id_cifrado: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Estadisticas {
    pub total_ids_originales: i64,
    pub total_monedas_cifradas: i64,
    pub monedas_minadas: i64,
    pub monedas_disponibles: i64,
    pub saldo_actual: i64,
    pub valor_por_moneda: i64,
}

impl Estadisticas {
    pub fn valor_total_sistema_usd(&self) -> f64 {
        (self.total_monedas_cifradas * self.valor_por_moneda) as f64 / 1000.0
    }
    
    pub fn valor_minado_usd(&self) -> f64 {
        (self.monedas_minadas * self.valor_por_moneda) as f64 / 1000.0
    }
    
    pub fn valor_disponible_usd(&self) -> f64 {
        (self.monedas_disponibles * self.valor_por_moneda) as f64 / 1000.0
    }
    
    pub fn saldo_actual_usd(&self) -> f64 {
        self.saldo_actual as f64 / 1000.0
    }
    
    pub fn porcentaje_minado(&self) -> f64 {
        if self.total_monedas_cifradas > 0 {
            (self.monedas_minadas as f64 / self.total_monedas_cifradas as f64) * 100.0
        } else {
            0.0
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HistorialTransaccion {
    pub fecha: String,
    pub id_moneda: i32,
    pub incremento: i64,
    pub valor_usd: String,
    pub saldo_nuevo: i64,
    pub saldo_nuevo_usd: String,
    pub tipo: String,
    pub moneda: String,
    pub id_original_preview: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SaldoResponse {
    pub saldo: i64,
    pub saldo_usd: f64,
    pub monedas_minadas: i64,
    pub ultima_actualizacion: String,
}