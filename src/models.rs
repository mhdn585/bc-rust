use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct MonedaPendiente {
    pub id: i32,
    pub id_cifrado: String,
    pub tabla: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Estadisticas {
    pub total_ids_originales: i64,
    pub total_monedas_cifradas: i64,
    pub monedas_minadas: i64,
    pub monedas_disponibles: i64,
    pub saldo_actual: i64,
}