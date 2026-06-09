use std::env;
use std::sync::Arc;
use sqlx::postgres::{PgPool, PgPoolOptions};
use sqlx::{Row, types::Json};
use dotenv::dotenv;
use chrono::Utc;
use once_cell::sync::OnceCell;
use crate::logs::{log_event, log_error};
use crate::models::{MonedaPendiente, Estadisticas};
use crate::crear_monedas::VALOR_MERCURY;

pub struct DatabasePool {
    pool: PgPool,
}

impl DatabasePool {
    async fn new() -> Result<Self, sqlx::Error> {
        dotenv().ok();
        let host = env::var("DB_HOST").unwrap_or_else(|_| "localhost".to_string());
        let port = env::var("DB_PORT").unwrap_or_else(|_| "5432".to_string());
        let database = env::var("DB_NAME").unwrap_or_else(|_| "monedas_db".to_string());
        let user = env::var("DB_USER").unwrap_or_else(|_| "postgres".to_string());
        let password = env::var("DB_PASSWORD").unwrap_or_else(|_| "".to_string());

        let database_url = format!(
            "postgres://{}:{}@{}:{}/{}",
            user, password, host, port, database
        );

        let pool = PgPoolOptions::new()
            .max_connections(20)
            .min_connections(1)
            .connect(&database_url)
            .await?;

        let _ = log_event("Pool de conexiones PostgreSQL inicializado para Mercury");
        Ok(DatabasePool { pool })
    }

    pub fn get_pool(&self) -> &PgPool {
        &self.pool
    }
}

static DB_POOL: OnceCell<Arc<DatabasePool>> = OnceCell::new();

pub async fn init_db_pool() -> Result<(), sqlx::Error> {
    dotenv().ok();
    let pool = Arc::new(DatabasePool::new().await?);
    let _ = DB_POOL.set(pool);
    Ok(())
}

pub fn get_pool() -> Arc<DatabasePool> {
    DB_POOL.get().expect("Database pool not initialized. Call init_db_pool() first.").clone()
}

pub async fn init_database() -> bool {
    dotenv().ok();
    
    if let Err(e) = init_db_pool().await {
        log_error(&format!("Error al inicializar pool de conexiones: {}", e));
        return false;
    }
    
    let pool = get_pool();

    let result = sqlx::query(
        "CREATE TABLE IF NOT EXISTS ids_originales (
            id SERIAL PRIMARY KEY,
            id_original TEXT NOT NULL UNIQUE,
            fecha_creacion TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )"
    ).execute(pool.get_pool()).await;

    if let Err(e) = result {
        log_error(&format!("Error al crear tabla ids_originales: {}", e));
        return false;
    }

    let result = sqlx::query(
        "CREATE TABLE IF NOT EXISTS monedas_cifradas (
            id SERIAL PRIMARY KEY,
            id_cifrado TEXT NOT NULL,
            estado BOOLEAN DEFAULT FALSE,
            fecha_creacion TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            fecha_minado TIMESTAMP NULL
        )"
    ).execute(pool.get_pool()).await;

    if let Err(e) = result {
        log_error(&format!("Error al crear tabla monedas_cifradas: {}", e));
        return false;
    }

    let result = sqlx::query(
        "CREATE TABLE IF NOT EXISTS saldo (
            id SERIAL PRIMARY KEY,
            saldo BIGINT DEFAULT 0,
            ultima_actualizacion TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            historial JSONB DEFAULT '[]'::jsonb
        )"
    ).execute(pool.get_pool()).await;

    if let Err(e) = result {
        log_error(&format!("Error al crear tabla saldo: {}", e));
        return false;
    }

    let _ = sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_monedas_estado ON monedas_cifradas(estado)"
    ).execute(pool.get_pool()).await;

    let _ = sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_monedas_fecha_creacion ON monedas_cifradas(fecha_creacion)"
    ).execute(pool.get_pool()).await;

    let _ = sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_ids_originales_fecha ON ids_originales(fecha_creacion)"
    ).execute(pool.get_pool()).await;

    let _ = sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_monedas_id ON monedas_cifradas(id)"
    ).execute(pool.get_pool()).await;

    let row: Option<(i64,)> = sqlx::query_as("SELECT COUNT(*) FROM saldo")
        .fetch_optional(pool.get_pool())
        .await
        .unwrap_or(None);

    let count = row.map(|r| r.0).unwrap_or(0);

    if count == 0 {
        let _ = sqlx::query(
            "INSERT INTO saldo (saldo, historial) VALUES (0, '[]'::jsonb)"
        ).execute(pool.get_pool()).await;
        let _ = log_event("Registro de saldo inicial creado para Mercury");
    }

    let _ = log_event("Base de datos Mercury inicializada correctamente");
    true
}

pub async fn insertar_id_original(id_original: &str) -> Option<i32> {
    let pool = get_pool();
    let row = sqlx::query(
        "INSERT INTO ids_originales (id_original) VALUES ($1) ON CONFLICT (id_original) DO NOTHING RETURNING id"
    )
    .bind(id_original)
    .fetch_optional(pool.get_pool())
    .await
    .unwrap_or(None);

    row.map(|r| r.get(0))
}

pub async fn insertar_moneda_cifrada(id_cifrado: &str, estado: bool) -> Option<i32> {
    let pool = get_pool();
    let row = sqlx::query(
        "INSERT INTO monedas_cifradas (id_cifrado, estado, fecha_creacion) VALUES ($1, $2, CURRENT_TIMESTAMP) RETURNING id"
    )
    .bind(id_cifrado)
    .bind(estado)
    .fetch_optional(pool.get_pool())
    .await
    .unwrap_or(None);

    row.map(|r| r.get(0))
}

pub async fn verificar_id_original_existe(id_original: &str) -> bool {
    let pool = get_pool();
    let row: Option<(bool,)> = sqlx::query_as(
        "SELECT EXISTS(SELECT 1 FROM ids_originales WHERE id_original = $1)"
    )
    .bind(id_original)
    .fetch_optional(pool.get_pool())
    .await
    .unwrap_or(None);

    row.map(|r| r.0).unwrap_or(false)
}

pub async fn obtener_siguiente_moneda_no_minada(limite: i64) -> Vec<MonedaPendiente> {
    let pool = get_pool();
    let rows = sqlx::query_as::<_, MonedaPendiente>(
        "SELECT id, id_cifrado FROM monedas_cifradas WHERE estado = FALSE ORDER BY id LIMIT $1"
    )
    .bind(limite)
    .fetch_all(pool.get_pool())
    .await
    .unwrap_or_default();

    rows
}

pub async fn actualizar_estado_moneda(moneda_id: i32, estado: bool) -> bool {
    let pool = get_pool();
    let result = sqlx::query(
        "UPDATE monedas_cifradas SET estado = $1, fecha_minado = CASE WHEN $2 THEN CURRENT_TIMESTAMP ELSE fecha_minado END WHERE id = $3"
    )
    .bind(estado)
    .bind(estado)
    .bind(moneda_id)
    .execute(pool.get_pool())
    .await;

    match result {
        Ok(res) => res.rows_affected() > 0,
        Err(e) => {
            log_error(&format!("Error al actualizar estado de moneda Mercury: {}", e));
            false
        }
    }
}

pub async fn obtener_saldo() -> Result<i64, sqlx::Error> {
    let pool = get_pool();
    let row: Option<(i64,)> = sqlx::query_as("SELECT saldo FROM saldo ORDER BY id DESC LIMIT 1")
        .fetch_optional(pool.get_pool())
        .await?;

    Ok(row.map(|r| r.0).unwrap_or(0))
}

pub async fn actualizar_saldo(incremento: i64, id_moneda: Option<i32>, id_original_preview: Option<&str>) -> Result<i64, sqlx::Error> {
    let pool = get_pool();

    let row: Option<(i64,)> = sqlx::query_as(
        "UPDATE saldo SET saldo = saldo + $1, ultima_actualizacion = CURRENT_TIMESTAMP WHERE id = (SELECT id FROM saldo ORDER BY id DESC LIMIT 1) RETURNING saldo"
    )
    .bind(incremento)
    .fetch_optional(pool.get_pool())
    .await?;

    let nuevo_saldo = row.map(|r| r.0).unwrap_or(0);

    if let Some(id_moneda_val) = id_moneda {
        let valor_usd = incremento as f64 / 1000.0;
        let mut registro = serde_json::json!({
            "fecha": Utc::now().to_rfc3339(),
            "id_moneda": id_moneda_val,
            "incremento": incremento,
            "valor_usd": format!("${:.3}", valor_usd),
            "saldo_nuevo": nuevo_saldo,
            "saldo_nuevo_usd": format!("${:.3}", nuevo_saldo as f64 / 1000.0),
            "tipo": "minado_exitoso",
            "moneda": "Mercury"
        });

        if let Some(preview) = id_original_preview {
            registro["id_original_preview"] = serde_json::json!(preview);
        }

        let _ = sqlx::query(
            "UPDATE saldo SET historial = historial || $1::jsonb WHERE id = (SELECT id FROM saldo ORDER BY id DESC LIMIT 1)"
        )
        .bind(Json(&vec![registro]))
        .execute(pool.get_pool())
        .await?;
    }

    Ok(nuevo_saldo)
}

pub async fn obtener_total_monedas() -> Result<i64, sqlx::Error> {
    let pool = get_pool();
    let row: Option<(i64,)> = sqlx::query_as("SELECT COUNT(*) FROM monedas_cifradas")
        .fetch_optional(pool.get_pool())
        .await?;

    Ok(row.map(|r| r.0).unwrap_or(0))
}

pub async fn obtener_monedas_minadas() -> Result<i64, sqlx::Error> {
    let pool = get_pool();
    let row: Option<(i64,)> = sqlx::query_as("SELECT COUNT(*) FROM monedas_cifradas WHERE estado = TRUE")
        .fetch_optional(pool.get_pool())
        .await?;

    Ok(row.map(|r| r.0).unwrap_or(0))
}

pub async fn obtener_monedas_disponibles() -> Result<i64, sqlx::Error> {
    let pool = get_pool();
    let row: Option<(i64,)> = sqlx::query_as("SELECT COUNT(*) FROM monedas_cifradas WHERE estado = FALSE")
        .fetch_optional(pool.get_pool())
        .await?;

    Ok(row.map(|r| r.0).unwrap_or(0))
}

pub async fn obtener_estadisticas_completas() -> Estadisticas {
    let pool = get_pool();

    let total_ids: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM ids_originales")
        .fetch_one(pool.get_pool())
        .await
        .unwrap_or(0);

    let total_monedas: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM monedas_cifradas")
        .fetch_one(pool.get_pool())
        .await
        .unwrap_or(0);

    let monedas_minadas: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM monedas_cifradas WHERE estado = TRUE")
        .fetch_one(pool.get_pool())
        .await
        .unwrap_or(0);

    let monedas_disponibles: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM monedas_cifradas WHERE estado = FALSE")
        .fetch_one(pool.get_pool())
        .await
        .unwrap_or(0);

    let saldo_actual: i64 = obtener_saldo().await.unwrap_or(0);

    Estadisticas {
        total_ids_originales: total_ids,
        total_monedas_cifradas: total_monedas,
        monedas_minadas,
        monedas_disponibles,
        saldo_actual,
        valor_por_moneda: VALOR_MERCURY,
    }
}

pub async fn reiniciar_base_datos() -> bool {
    let pool = get_pool();

    let _ = sqlx::query("TRUNCATE TABLE ids_originales RESTART IDENTITY CASCADE")
        .execute(pool.get_pool())
        .await;

    let _ = sqlx::query("TRUNCATE TABLE monedas_cifradas RESTART IDENTITY CASCADE")
        .execute(pool.get_pool())
        .await;

    let _ = sqlx::query("TRUNCATE TABLE saldo RESTART IDENTITY CASCADE")
        .execute(pool.get_pool())
        .await;

    let _ = sqlx::query("INSERT INTO saldo (saldo, historial) VALUES (0, '[]'::jsonb)")
        .execute(pool.get_pool())
        .await;

    let _ = log_event("Base de datos Mercury reiniciada completamente");
    true
}

pub async fn verificar_conexion() -> bool {
    if let Ok(()) = init_db_pool().await {
        let pool = get_pool();
        match sqlx::query("SELECT 1").fetch_one(pool.get_pool()).await {
            Ok(_) => true,
            Err(e) => {
                log_error(&format!("Error al verificar conexion a PostgreSQL: {}", e));
                false
            }
        }
    } else {
        false
    }
}

pub async fn cerrar_pool() {
    if let Some(pool) = DB_POOL.get() {
        pool.pool.close().await;
        let _ = log_event("Pool de conexiones cerrado");
    }
}