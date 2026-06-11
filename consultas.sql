-- =====================================================
-- CONSULTAS PARA EL SISTEMA MERCURY
-- Base de datos: PostgreSQL
-- Moneda: Mercury (valor $67.998 USD cada una)
-- Cifrado: AES-256-GCM
-- =====================================================

-- =====================================================
-- 1. ESTADÍSTICAS BÁSICAS DEL SISTEMA
-- =====================================================

-- 1.1 Resumen general (lo más importante)
SELECT 
    (SELECT COUNT(*) FROM ids_originales) AS total_ids_originales,
    (SELECT COUNT(*) FROM monedas_cifradas) AS total_monedas,
    (SELECT COUNT(*) FROM monedas_cifradas WHERE porcentaje_minado >= 99.9999) AS monedas_minadas_completas,
    (SELECT COUNT(*) FROM monedas_cifradas WHERE porcentaje_minado > 0 AND porcentaje_minado < 99.9999) AS monedas_minadas_parciales,
    (SELECT COUNT(*) FROM monedas_cifradas WHERE porcentaje_minado < 0.0001) AS monedas_disponibles,
    (SELECT ROUND(COALESCE(saldo, 0)::numeric / 1000, 3) FROM saldo ORDER BY id DESC LIMIT 1) AS saldo_actual_usd;

-- 1.2 Porcentaje de minado del sistema
SELECT 
    COUNT(*) AS total_monedas,
    SUM(CASE WHEN porcentaje_minado >= 99.9999 THEN 1 ELSE 0 END) AS completas,
    SUM(CASE WHEN porcentaje_minado > 0 AND porcentaje_minado < 99.9999 THEN 1 ELSE 0 END) AS parciales,
    SUM(CASE WHEN porcentaje_minado < 0.0001 THEN 1 ELSE 0 END) AS disponibles,
    ROUND(100.0 * SUM(porcentaje_minado) / (COUNT(*) * 100.0), 2) AS porcentaje_total_minado
FROM monedas_cifradas;

-- =====================================================
-- 2. CONSULTAS SOBRE MONEDAS
-- =====================================================

-- 2.1 Ver las últimas 10 monedas creadas
SELECT 
    id, 
    LEFT(id_cifrado, 50) AS id_cifrado_preview, 
    ROUND(porcentaje_minado::numeric, 4) AS porcentaje,
    fecha_creacion, 
    fecha_minado
FROM monedas_cifradas
ORDER BY id DESC
LIMIT 10;

-- 2.2 Ver las últimas 10 monedas minadas completamente
SELECT 
    id, 
    LEFT(id_cifrado, 50) AS id_cifrado_preview, 
    fecha_creacion, 
    fecha_minado
FROM monedas_cifradas
WHERE porcentaje_minado >= 99.9999
ORDER BY fecha_minado DESC
LIMIT 10;

-- 2.3 Ver las monedas disponibles para minar (primeras 10, menor porcentaje primero)
SELECT 
    id, 
    LEFT(id_cifrado, 50) AS id_cifrado_preview, 
    ROUND(porcentaje_minado::numeric, 4) AS porcentaje_actual,
    ROUND((100 - porcentaje_minado)::numeric, 4) AS porcentaje_restante,
    fecha_creacion
FROM monedas_cifradas
WHERE porcentaje_minado < 99.9999
ORDER BY porcentaje_minado ASC
LIMIT 10;

-- 2.4 Contar monedas por rango de porcentaje de minado
SELECT 
    CASE 
        WHEN porcentaje_minado >= 99.9999 THEN '100% Completas'
        WHEN porcentaje_minado >= 75 THEN '75% - 99%'
        WHEN porcentaje_minado >= 50 THEN '50% - 74%'
        WHEN porcentaje_minado >= 25 THEN '25% - 49%'
        WHEN porcentaje_minado >= 1 THEN '1% - 24%'
        ELSE '0% No minadas'
    END AS rango,
    COUNT(*) AS cantidad,
    ROUND(AVG(porcentaje_minado)::numeric, 2) AS promedio_porcentaje,
    ROUND((COUNT(*) * 67.998)::numeric, 3) AS valor_total_usd
FROM monedas_cifradas
GROUP BY 
    CASE 
        WHEN porcentaje_minado >= 99.9999 THEN '100% Completas'
        WHEN porcentaje_minado >= 75 THEN '75% - 99%'
        WHEN porcentaje_minado >= 50 THEN '50% - 74%'
        WHEN porcentaje_minado >= 25 THEN '25% - 49%'
        WHEN porcentaje_minado >= 1 THEN '1% - 24%'
        ELSE '0% No minadas'
    END
ORDER BY 
    CASE 
        WHEN porcentaje_minado >= 99.9999 THEN 1
        WHEN porcentaje_minado >= 75 THEN 2
        WHEN porcentaje_minado >= 50 THEN 3
        WHEN porcentaje_minado >= 25 THEN 4
        WHEN porcentaje_minado >= 1 THEN 5
        ELSE 6
    END;

-- 2.5 Monedas con minado en progreso (parciales, ordenadas por mayor porcentaje)
SELECT 
    id, 
    ROUND(porcentaje_minado::numeric, 4) AS porcentaje_actual,
    ROUND((porcentaje_minado / 100.0 * 67.998)::numeric, 4) AS valor_obtenido_usd,
    ROUND(((100 - porcentaje_minado) / 100.0 * 67.998)::numeric, 4) AS valor_restante_usd,
    fecha_creacion
FROM monedas_cifradas 
WHERE porcentaje_minado > 0 AND porcentaje_minado < 99.9999
ORDER BY porcentaje_minado DESC
LIMIT 20;

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

-- 4.1 Ver el saldo actual en centavos y USD
SELECT 
    saldo AS saldo_centavos,
    ROUND(saldo::numeric / 1000, 3) AS saldo_usd,
    ultima_actualizacion
FROM saldo 
ORDER BY id DESC 
LIMIT 1;

-- 4.2 Ver el historial completo de minado (último registro con JSON)
SELECT 
    id,
    saldo,
    ROUND(saldo::numeric / 1000, 3) AS saldo_usd,
    ultima_actualizacion,
    jsonb_pretty(historial) AS historial_detalle
FROM saldo
ORDER BY id DESC
LIMIT 1;

-- 4.3 Ver la cantidad total de transacciones registradas
SELECT 
    jsonb_array_length(historial) AS total_transacciones
FROM saldo
ORDER BY id DESC
LIMIT 1;

-- 4.4 Ver las últimas 10 transacciones del historial
SELECT 
    jsonb_array_elements(historial) AS transaccion
FROM saldo
ORDER BY id DESC
LIMIT 1;

-- =====================================================
-- 5. VERIFICACIONES DE INTEGRIDAD
-- =====================================================

-- 5.1 Verificación completa de integridad
SELECT 
    (SELECT COUNT(*) FROM ids_originales) AS total_ids,
    (SELECT COUNT(*) FROM monedas_cifradas) AS total_monedas,
    (SELECT COUNT(*) FROM monedas_cifradas WHERE porcentaje_minado >= 99.9999) AS completas,
    (SELECT COUNT(*) FROM monedas_cifradas WHERE porcentaje_minado > 0 AND porcentaje_minado < 99.9999) AS parciales,
    (SELECT COUNT(*) FROM monedas_cifradas WHERE porcentaje_minado < 0.0001) AS disponibles,
    CASE 
        WHEN (SELECT COUNT(*) FROM ids_originales) = (SELECT COUNT(*) FROM monedas_cifradas) 
        THEN 'OK - Integridad correcta'
        ELSE 'ERROR - Inconsistencia de datos'
    END AS estado_integridad,
    CASE 
        WHEN (SELECT COUNT(*) FROM monedas_cifradas) = 100000 
        THEN 'OK - Cantidad correcta (100,000)'
        ELSE 'ERROR - No son 100,000 monedas'
    END AS verificacion_cantidad;

-- 5.2 Verificar que no hay porcentajes negativos o mayores a 100
SELECT COUNT(*) AS valores_invalidos
FROM monedas_cifradas 
WHERE porcentaje_minado < 0 OR porcentaje_minado > 100;

-- 5.3 Ver monedas con fecha de minado pero porcentaje no completado (inconsistencia)
SELECT id, porcentaje_minado, fecha_minado
FROM monedas_cifradas
WHERE fecha_minado IS NOT NULL AND porcentaje_minado < 99.9999;

-- 5.4 Ver monedas con porcentaje completado pero sin fecha de minado (inconsistencia)
SELECT id, porcentaje_minado, fecha_minado
FROM monedas_cifradas
WHERE porcentaje_minado >= 99.9999 AND fecha_minado IS NULL;

-- =====================================================
-- 6. CONSULTAS DE VALOR Y FINANZAS
-- =====================================================

-- 6.1 Valor total del sistema
SELECT 
    ROUND(COUNT(*) * 67.998, 3) AS valor_total_sistema_usd,
    ROUND(SUM(CASE WHEN porcentaje_minado >= 99.9999 THEN 67.998 ELSE 0 END), 3) AS valor_minado_completo_usd,
    ROUND(SUM(porcentaje_minado / 100.0 * 67.998), 3) AS valor_obtenido_real_usd,
    ROUND(SUM((100 - porcentaje_minado) / 100.0 * 67.998), 3) AS valor_por_obtener_usd
FROM monedas_cifradas;

-- 6.2 Progreso financiero del sistema
SELECT 
    ROUND((SUM(porcentaje_minado) / (COUNT(*) * 100.0) * 100)::numeric, 2) AS porcentaje_total_minado,
    ROUND((SUM(porcentaje_minado) / 100.0 * 67.998)::numeric, 2) AS valor_total_obtenido_usd,
    ROUND((COUNT(*) * 67.998)::numeric, 2) AS valor_maximo_posible_usd,
    ROUND((SUM(porcentaje_minado) / 100.0 * 67.998 * 1000)::numeric, 0) AS valor_obtenido_centavos
FROM monedas_cifradas;

-- 6.3 Equivalencia del saldo actual en monedas
SELECT 
    ROUND(saldo::numeric / 1000, 3) AS saldo_usd,
    FLOOR(saldo::numeric / 67.998) AS monedas_completas,
    ROUND((saldo::numeric % 67.998) / 67.998 * 100, 2) AS porcentaje_moneda_parcial
FROM saldo 
ORDER BY id DESC 
LIMIT 1;

-- =====================================================
-- 7. CONSULTAS DE AUDITORÍA
-- =====================================================

-- 7.1 Antigüedad de las monedas disponibles (las más viejas primero)
SELECT 
    id, 
    LEFT(id_cifrado, 50) AS preview, 
    fecha_creacion, 
    EXTRACT(DAY FROM (NOW() - fecha_creacion)) AS dias_antiguedad
FROM monedas_cifradas
WHERE porcentaje_minado < 0.0001
ORDER BY fecha_creacion
LIMIT 10;

-- 7.2 Monedas minadas recientemente (últimas 10)
SELECT 
    id, 
    fecha_minado,
    ROUND(porcentaje_minado::numeric, 2) AS porcentaje_final
FROM monedas_cifradas 
WHERE porcentaje_minado >= 99.9999 AND fecha_minado IS NOT NULL
ORDER BY fecha_minado DESC 
LIMIT 20;

-- 7.3 Velocidad de minado (promedio de monedas completas por día)
SELECT 
    COUNT(*) AS monedas_minadas,
    EXTRACT(DAY FROM (MAX(fecha_minado) - MIN(fecha_minado))) + 1 AS dias_transcurridos,
    ROUND(COUNT(*) / NULLIF(EXTRACT(DAY FROM (MAX(fecha_minado) - MIN(fecha_minado))) + 1, 0), 2) AS monedas_por_dia
FROM monedas_cifradas
WHERE porcentaje_minado >= 99.9999 AND fecha_minado IS NOT NULL;

-- 7.4 Última actualización del sistema
SELECT 
    'saldo' AS tabla,
    ultima_actualizacion AS ultima_modificacion
FROM saldo 
ORDER BY id DESC 
LIMIT 1
UNION ALL
SELECT 
    'monedas_cifradas' AS tabla,
    MAX(fecha_minado) AS ultima_modificacion
FROM monedas_cifradas
WHERE fecha_minado IS NOT NULL;

-- =====================================================
-- 8. CONSULTAS DE MANTENIMIENTO
-- =====================================================

-- 8.1 Ver el tamaño de las tablas
SELECT 
    table_name,
    pg_size_pretty(pg_total_relation_size(quote_ident(table_name))) AS tamaño_total,
    pg_size_pretty(pg_relation_size(quote_ident(table_name))) AS tamaño_datos,
    pg_size_pretty(pg_indexes_size(quote_ident(table_name))) AS tamaño_indices
FROM information_schema.tables
WHERE table_schema = 'public'
AND table_type = 'BASE TABLE'
ORDER BY pg_total_relation_size(quote_ident(table_name)) DESC;

-- 8.2 Ver estadísticas de las tablas
SELECT 
    relname AS tabla,
    n_live_tup AS filas_vivas,
    n_dead_tup AS filas_muertas,
    last_vacuum,
    last_autovacuum,
    last_analyze
FROM pg_stat_user_tables
WHERE relname IN ('monedas_cifradas', 'ids_originales', 'saldo');

-- 8.3 Ver los índices existentes
SELECT 
    tablename,
    indexname,
    indexdef
FROM pg_indexes
WHERE schemaname = 'public'
ORDER BY tablename, indexname;

-- =====================================================
-- 9. CONSULTAS PARA DEPURACIÓN DE PROBLEMAS
-- =====================================================

-- 9.1 Ver si hay monedas con ID cifrado vacío o nulo
SELECT id, id_cifrado, porcentaje_minado
FROM monedas_cifradas
WHERE id_cifrado IS NULL OR id_cifrado = '';

-- 9.2 Ver monedas con longitud anormal de ID cifrado
SELECT 
    id,
    LENGTH(id_cifrado) AS longitud,
    LEFT(id_cifrado, 50) AS preview
FROM monedas_cifradas
WHERE LENGTH(id_cifrado) < 100 OR LENGTH(id_cifrado) > 2000
LIMIT 10;

-- 9.3 Ver IDs originales con longitud anormal
SELECT 
    id,
    LENGTH(id_original) AS longitud,
    LEFT(id_original, 50) AS preview
FROM ids_originales
WHERE LENGTH(id_original) != 1024
LIMIT 10;

-- 9.4 Verificar que no hay monedas sin ID original correspondiente
-- (Esto es solo informativo, no hay FK directa)
SELECT 
    (SELECT COUNT(*) FROM monedas_cifradas) AS total_monedas,
    (SELECT COUNT(*) FROM ids_originales) AS total_ids,
    (SELECT COUNT(*) FROM monedas_cifradas) - (SELECT COUNT(*) FROM ids_originales) AS diferencia;

-- =====================================================
-- 10. CONSULTAS RÁPIDAS (LAS MÁS ÚTILES)
-- =====================================================

-- 10.1 ¿Cuántas monedas hay en total?
SELECT COUNT(*) AS total_monedas FROM monedas_cifradas;

-- 10.2 ¿Cuántas monedas están minadas completamente?
SELECT COUNT(*) AS minadas_completas FROM monedas_cifradas WHERE porcentaje_minado >= 99.9999;

-- 10.3 ¿Cuántas monedas están minadas parcialmente?
SELECT COUNT(*) AS minadas_parciales FROM monedas_cifradas WHERE porcentaje_minado > 0 AND porcentaje_minado < 99.9999;

-- 10.4 ¿Cuántas monedas faltan por minar?
SELECT COUNT(*) AS disponibles FROM monedas_cifradas WHERE porcentaje_minado < 0.0001;

-- 10.5 ¿Cuál es el saldo actual en USD?
SELECT ROUND(COALESCE(saldo, 0)::numeric / 1000, 3) AS saldo_usd FROM saldo ORDER BY id DESC LIMIT 1;

-- 10.6 ¿Hay algún error de integridad?
SELECT 
    (SELECT COUNT(*) FROM monedas_cifradas WHERE porcentaje_minado >= 99.9999 AND fecha_minado IS NULL) AS completas_sin_fecha,
    (SELECT COUNT(*) FROM monedas_cifradas WHERE porcentaje_minado < 99.9999 AND fecha_minado IS NOT NULL) AS parciales_con_fecha,
    (SELECT COUNT(*) FROM ids_originales WHERE id_original IS NULL OR id_original = '') AS ids_invalidos,
    (SELECT COUNT(*) FROM monedas_cifradas WHERE id_cifrado IS NULL OR id_cifrado = '') AS monedas_sin_cifrado,
    (SELECT COUNT(*) FROM monedas_cifradas WHERE porcentaje_minado < 0 OR porcentaje_minado > 100) AS porcentajes_invalidos;

-- 10.7 Resumen rápido en una sola línea
SELECT 
    'Mercury' AS moneda,
    (SELECT COUNT(*) FROM monedas_cifradas) AS total,
    (SELECT COUNT(*) FROM monedas_cifradas WHERE porcentaje_minado >= 99.9999) AS completas,
    (SELECT COUNT(*) FROM monedas_cifradas WHERE porcentaje_minado > 0 AND porcentaje_minado < 99.9999) AS parciales,
    (SELECT COUNT(*) FROM monedas_cifradas WHERE porcentaje_minado < 0.0001) AS disponibles,
    (SELECT ROUND(COALESCE(saldo, 0)::numeric / 1000, 3) FROM saldo ORDER BY id DESC LIMIT 1) AS saldo_usd,
    ROUND((SELECT SUM(porcentaje_minado) / (SELECT COUNT(*) * 100.0) * 100 FROM monedas_cifradas)::numeric, 2) AS porcentaje_minado_total;

-- =====================================================
-- 11. COMANDOS PARA CONECTARSE A LA BASE DE DATOS
-- =====================================================

-- Conexión estándar
-- psql -h localhost -p 5432 -U postgres -d monedas_db

-- Conexión con contraseña
-- PGPASSWORD="tu_contraseña" psql -h localhost -p 5432 -U postgres -d monedas_db

-- Conexión desde pgAdmin o DBeaver
-- Host: localhost
-- Puerto: 5432
-- Base de datos: monedas_db
-- Usuario: postgres
-- Contraseña: la que configuraste en .env

-- =====================================================
-- NOTAS IMPORTANTES
-- =====================================================

-- 1. Todas las consultas son SOLO LECTURA (SELECT), no modifican datos
-- 2. El valor de cada Mercury es $67.998 USD
-- 3. El saldo se almacena en centavos (ej: 67998 = $67.998)
-- 4. El porcentaje de minado usa DOUBLE PRECISION con 4 decimales de precision
-- 5. Una moneda se considera "completa" cuando porcentaje_minado >= 99.9999
-- 6. Una moneda se considera "parcial" cuando porcentaje_minado > 0 y < 99.9999
-- 7. Una moneda se considera "disponible" cuando porcentaje_minado < 0.0001
-- 8. Para migrar la base de datos existente se usó: 
--    ALTER TABLE monedas_cifradas ALTER COLUMN porcentaje_minado TYPE DOUBLE PRECISION;