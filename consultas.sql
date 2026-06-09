-- =====================================================
-- CONSULTAS PARA EL SISTEMA DE MONEDAS
-- Base de datos: PostgreSQL
-- Proyecto: bc-rust (Blockchain en Rust)
-- =====================================================

-- =====================================================
-- 1. ESTADÍSTICAS BÁSICAS DEL SISTEMA
-- =====================================================

-- 1.1 Resumen general (lo más importante)
SELECT 
    (SELECT COUNT(*) FROM ids_originales) AS total_ids_originales,
    (SELECT COUNT(*) FROM monedas_cifradas) AS total_monedas_cifradas,
    (SELECT COUNT(*) FROM monedas_cifradas WHERE estado = true) AS monedas_minadas,
    (SELECT COUNT(*) FROM monedas_cifradas WHERE estado = false) AS monedas_disponibles,
    (SELECT COALESCE(saldo, 0) FROM saldo ORDER BY id DESC LIMIT 1) AS saldo_actual;

-- 1.2 Porcentaje de minado
SELECT 
    COUNT(*) AS total_monedas,
    SUM(CASE WHEN estado = true THEN 1 ELSE 0 END) AS minadas,
    SUM(CASE WHEN estado = false THEN 1 ELSE 0 END) AS disponibles,
    ROUND(100.0 * SUM(CASE WHEN estado = true THEN 1 ELSE 0 END) / COUNT(*), 2) AS porcentaje_minado
FROM monedas_cifradas;

-- =====================================================
-- 2. CONSULTAS SOBRE MONEDAS
-- =====================================================

-- 2.1 Ver las últimas 10 monedas creadas
SELECT id, LEFT(id_cifrado, 50) AS id_cifrado_preview, estado, fecha_creacion, fecha_minado
FROM monedas_cifradas
ORDER BY id DESC
LIMIT 10;

-- 2.2 Ver las últimas 10 monedas minadas
SELECT id, LEFT(id_cifrado, 50) AS id_cifrado_preview, fecha_creacion, fecha_minado
FROM monedas_cifradas
WHERE estado = true
ORDER BY fecha_minado DESC
LIMIT 10;

-- 2.3 Ver las monedas disponibles para minar (primeras 10)
SELECT id, LEFT(id_cifrado, 50) AS id_cifrado_preview, fecha_creacion
FROM monedas_cifradas
WHERE estado = false
ORDER BY id
LIMIT 10;

-- 2.4 Contar monedas por rango de fechas (creación)
SELECT 
    DATE(fecha_creacion) AS fecha,
    COUNT(*) AS monedas_creadas
FROM monedas_cifradas
GROUP BY DATE(fecha_creacion)
ORDER BY fecha DESC
LIMIT 10;

-- 2.5 Contar monedas por rango de fechas (minado)
SELECT 
    DATE(fecha_minado) AS fecha,
    COUNT(*) AS monedas_minadas
FROM monedas_cifradas
WHERE estado = true AND fecha_minado IS NOT NULL
GROUP BY DATE(fecha_minado)
ORDER BY fecha DESC
LIMIT 10;

-- =====================================================
-- 3. CONSULTAS SOBRE IDs ORIGINALES
-- =====================================================

-- 3.1 Ver los primeros 10 IDs originales (preview)
SELECT id, LEFT(id_original, 50) AS id_original_preview, fecha_creacion
FROM ids_originales
ORDER BY id
LIMIT 10;

-- 3.2 Ver los últimos 10 IDs originales creados
SELECT id, LEFT(id_original, 100) AS id_original_preview, fecha_creacion
FROM ids_originales
ORDER BY id DESC
LIMIT 10;

-- 3.3 Verificar si hay IDs originales duplicados (debería dar 0)
SELECT id_original, COUNT(*) AS duplicados
FROM ids_originales
GROUP BY id_original
HAVING COUNT(*) > 1;

-- 3.4 Contar IDs originales por fecha
SELECT 
    DATE(fecha_creacion) AS fecha,
    COUNT(*) AS ids_creados
FROM ids_originales
GROUP BY DATE(fecha_creacion)
ORDER BY fecha DESC;

-- =====================================================
-- 4. CONSULTAS SOBRE SALDO
-- =====================================================

-- 4.1 Ver el saldo actual
SELECT saldo, ultima_actualizacion FROM saldo ORDER BY id DESC LIMIT 1;

-- 4.2 Ver el historial completo de minado (JSON)
SELECT 
    id,
    saldo,
    ultima_actualizacion,
    historial::TEXT AS historial_json
FROM saldo
ORDER BY id DESC
LIMIT 1;

-- 4.3 Ver las últimas 10 transacciones del historial (si existe)
SELECT 
    jsonb_array_elements(historial) AS transaccion
FROM saldo
ORDER BY id DESC
LIMIT 1;

-- 4.4 Ver el histórico de saldo (todas las actualizaciones, no solo la última)
-- Nota: como solo hay una fila con historial JSON, esto expande todo
SELECT 
    id,
    saldo,
    ultima_actualizacion,
    jsonb_array_length(historial) AS total_movimientos
FROM saldo
ORDER BY id DESC;

-- =====================================================
-- 5. VERIFICACIONES DE INTEGRIDAD
-- =====================================================

-- 5.1 Verificar que no hay monedas con estado NULL
SELECT COUNT(*) FROM monedas_cifradas WHERE estado IS NULL;

-- 5.2 Verificar que todas las monedas cifradas tienen un ID original correspondiente
-- (Esto es solo informativo, no hay FK directa)
SELECT 
    (SELECT COUNT(*) FROM monedas_cifradas) AS total_monedas,
    (SELECT COUNT(*) FROM ids_originales) AS total_ids,
    (SELECT COUNT(*) FROM monedas_cifradas) - (SELECT COUNT(*) FROM ids_originales) AS diferencia;

-- 5.3 Ver monedas que fueron minadas pero no tienen fecha de minado (inconsistencia)
SELECT id, estado, fecha_minado
FROM monedas_cifradas
WHERE estado = true AND fecha_minado IS NULL;

-- 5.4 Ver monedas con fecha de minado pero estado false (inconsistencia)
SELECT id, estado, fecha_minado
FROM monedas_cifradas
WHERE estado = false AND fecha_minado IS NOT NULL;

-- =====================================================
-- 6. CONSULTAS DE AUDITORÍA
-- =====================================================

-- 6.1 Ver la antigüedad de las monedas disponibles (las más viejas primero)
SELECT id, LEFT(id_cifrado, 50), fecha_creacion, 
       EXTRACT(DAY FROM (NOW() - fecha_creacion)) AS dias_antiguedad
FROM monedas_cifradas
WHERE estado = false
ORDER BY fecha_creacion
LIMIT 10;

-- 6.2 Velocidad de minado (promedio de monedas por día)
SELECT 
    COUNT(*) AS monedas_minadas,
    EXTRACT(DAY FROM (MAX(fecha_minado) - MIN(fecha_minado))) AS dias_transcurridos,
    ROUND(COUNT(*) / NULLIF(EXTRACT(DAY FROM (MAX(fecha_minado) - MIN(fecha_minado))), 0), 2) AS monedas_por_dia
FROM monedas_cifradas
WHERE estado = true AND fecha_minado IS NOT NULL;

-- 6.3 Ver la última vez que se actualizó el saldo
SELECT ultima_actualizacion, saldo
FROM saldo
ORDER BY id DESC
LIMIT 1;

-- =====================================================
-- 7. CONSULTAS DE MANTENIMIENTO (SOLO LECTURA)
-- =====================================================

-- 7.1 Ver el tamaño de las tablas
SELECT 
    table_name,
    pg_size_pretty(pg_total_relation_size(quote_ident(table_name))) AS tamaño
FROM information_schema.tables
WHERE table_schema = 'public'
AND table_type = 'BASE TABLE'
ORDER BY pg_total_relation_size(quote_ident(table_name)) DESC;

-- 7.2 Ver estadísticas de la tabla monedas_cifradas
SELECT 
    relname AS tabla,
    n_live_tup AS filas_vivas,
    n_dead_tup AS filas_muertas,
    last_vacuum,
    last_autovacuum,
    last_analyze
FROM pg_stat_user_tables
WHERE relname IN ('monedas_cifradas', 'ids_originales', 'saldo');

-- 7.3 Ver los índices existentes
SELECT 
    tablename,
    indexname,
    indexdef
FROM pg_indexes
WHERE schemaname = 'public'
ORDER BY tablename, indexname;

-- =====================================================
-- 8. CONSULTAS PARA DEPURACIÓN DE PROBLEMAS
-- =====================================================

-- 8.1 Ver si hay monedas que nunca podrán ser minadas (corruptas)
-- Monedas que tienen estado false pero ID cifrado vacío o nulo
SELECT id, id_cifrado, estado
FROM monedas_cifradas
WHERE (id_cifrado IS NULL OR id_cifrado = '') AND estado = false;

-- 8.2 Ver monedas con longitud anormal de ID cifrado (debería ser siempre la misma)
SELECT 
    id,
    LENGTH(id_cifrado) AS longitud,
    LEFT(id_cifrado, 50) AS preview
FROM monedas_cifradas
WHERE LENGTH(id_cifrado) < 100 OR LENGTH(id_cifrado) > 2000
LIMIT 10;

-- 8.3 Ver el progreso exacto (cuántas faltan para terminar)
SELECT 
    (SELECT COUNT(*) FROM monedas_cifradas WHERE estado = false) AS monedas_faltantes,
    (SELECT COUNT(*) FROM monedas_cifradas WHERE estado = true) AS monedas_completadas,
    ROUND(100.0 * (SELECT COUNT(*) FROM monedas_cifradas WHERE estado = true) / 
          NULLIF((SELECT COUNT(*) FROM monedas_cifradas), 0), 2) AS porcentaje_completado;

-- =====================================================
-- 9. CONSULTAS RÁPIDAS (LAS MÁS ÚTILES)
-- =====================================================

-- 9.1 ¿Cuántas monedas hay en total?
SELECT COUNT(*) AS total_monedas FROM monedas_cifradas;

-- 9.2 ¿Cuántas monedas están minadas?
SELECT COUNT(*) AS minadas FROM monedas_cifradas WHERE estado = true;

-- 9.3 ¿Cuántas monedas faltan por minar?
SELECT COUNT(*) AS disponibles FROM monedas_cifradas WHERE estado = false;

-- 9.4 ¿Cuál es el saldo actual?
SELECT saldo FROM saldo ORDER BY id DESC LIMIT 1;

-- 9.5 ¿Hay algún error de integridad?
SELECT 
    (SELECT COUNT(*) FROM monedas_cifradas WHERE estado = true AND fecha_minado IS NULL) AS monedas_minadas_sin_fecha,
    (SELECT COUNT(*) FROM monedas_cifradas WHERE estado = false AND fecha_minado IS NOT NULL) AS monedas_no_minadas_con_fecha,
    (SELECT COUNT(*) FROM ids_originales WHERE id_original IS NULL OR id_original = '') AS ids_originales_invalidos,
    (SELECT COUNT(*) FROM monedas_cifradas WHERE id_cifrado IS NULL OR id_cifrado = '') AS monedas_sin_cifrado;

-- =====================================================
-- 10. RESET DE SECUENCIAS (si reiniciaste y algo no está bien)
-- NOTA: ESTAS CONSULTAS MODIFICAN DATOS, ÚSALAS CON CUIDADO
-- =====================================================

-- 10.1 Ver los valores actuales de las secuencias
SELECT 
    sequence_name,
    last_value,
    is_called
FROM information_schema.sequences
WHERE sequence_schema = 'public';

-- 10.2 Reiniciar secuencias a 1 (solo si limpiaste las tablas)
-- ALTER SEQUENCE ids_originales_id_seq RESTART WITH 1;
-- ALTER SEQUENCE monedas_cifradas_id_seq RESTART WITH 1;
-- ALTER SEQUENCE saldo_id_seq RESTART WITH 1;

-- =====================================================
-- 11. COMANDO PARA CONECTARSE
-- =====================================================

psql -h localhost -p 5432 -U postgres -d monedas_db

PGPASSWORD="tu_contraseña" psql -h localhost -p 5432 -U postgres -d monedas_db