--
-- PostgreSQL database dump
--

\restrict zaASbSF0isme7iuaDeYQCMNOhgum1BqGvj7hCUI9rmhFvNiiCH1aSXB3je7pKHN

-- Dumped from database version 16.14 (Ubuntu 16.14-0ubuntu0.24.04.1)
-- Dumped by pg_dump version 16.14 (Ubuntu 16.14-0ubuntu0.24.04.1)

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: ids_originales; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.ids_originales (
    id integer NOT NULL,
    id_original text NOT NULL,
    fecha_creacion timestamp without time zone DEFAULT CURRENT_TIMESTAMP
);


--
-- Name: ids_originales_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.ids_originales_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: ids_originales_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.ids_originales_id_seq OWNED BY public.ids_originales.id;


--
-- Name: monedas_00; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.monedas_00 (
    id integer NOT NULL,
    id_cifrado text NOT NULL,
    estado boolean DEFAULT false,
    fecha_creacion timestamp without time zone DEFAULT CURRENT_TIMESTAMP,
    fecha_minado timestamp without time zone
);


--
-- Name: monedas_00_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.monedas_00_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: monedas_00_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.monedas_00_id_seq OWNED BY public.monedas_00.id;


--
-- Name: monedas_01; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.monedas_01 (
    id integer NOT NULL,
    id_cifrado text NOT NULL,
    estado boolean DEFAULT false,
    fecha_creacion timestamp without time zone DEFAULT CURRENT_TIMESTAMP,
    fecha_minado timestamp without time zone
);


--
-- Name: monedas_01_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.monedas_01_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: monedas_01_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.monedas_01_id_seq OWNED BY public.monedas_01.id;


--
-- Name: monedas_02; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.monedas_02 (
    id integer NOT NULL,
    id_cifrado text NOT NULL,
    estado boolean DEFAULT false,
    fecha_creacion timestamp without time zone DEFAULT CURRENT_TIMESTAMP,
    fecha_minado timestamp without time zone
);


--
-- Name: monedas_02_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.monedas_02_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: monedas_02_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.monedas_02_id_seq OWNED BY public.monedas_02.id;


--
-- Name: monedas_03; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.monedas_03 (
    id integer NOT NULL,
    id_cifrado text NOT NULL,
    estado boolean DEFAULT false,
    fecha_creacion timestamp without time zone DEFAULT CURRENT_TIMESTAMP,
    fecha_minado timestamp without time zone
);


--
-- Name: monedas_03_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.monedas_03_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: monedas_03_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.monedas_03_id_seq OWNED BY public.monedas_03.id;


--
-- Name: monedas_04; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.monedas_04 (
    id integer NOT NULL,
    id_cifrado text NOT NULL,
    estado boolean DEFAULT false,
    fecha_creacion timestamp without time zone DEFAULT CURRENT_TIMESTAMP,
    fecha_minado timestamp without time zone
);


--
-- Name: monedas_04_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.monedas_04_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: monedas_04_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.monedas_04_id_seq OWNED BY public.monedas_04.id;


--
-- Name: monedas_05; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.monedas_05 (
    id integer NOT NULL,
    id_cifrado text NOT NULL,
    estado boolean DEFAULT false,
    fecha_creacion timestamp without time zone DEFAULT CURRENT_TIMESTAMP,
    fecha_minado timestamp without time zone
);


--
-- Name: monedas_05_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.monedas_05_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: monedas_05_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.monedas_05_id_seq OWNED BY public.monedas_05.id;


--
-- Name: monedas_06; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.monedas_06 (
    id integer NOT NULL,
    id_cifrado text NOT NULL,
    estado boolean DEFAULT false,
    fecha_creacion timestamp without time zone DEFAULT CURRENT_TIMESTAMP,
    fecha_minado timestamp without time zone
);


--
-- Name: monedas_06_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.monedas_06_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: monedas_06_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.monedas_06_id_seq OWNED BY public.monedas_06.id;


--
-- Name: monedas_07; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.monedas_07 (
    id integer NOT NULL,
    id_cifrado text NOT NULL,
    estado boolean DEFAULT false,
    fecha_creacion timestamp without time zone DEFAULT CURRENT_TIMESTAMP,
    fecha_minado timestamp without time zone
);


--
-- Name: monedas_07_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.monedas_07_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: monedas_07_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.monedas_07_id_seq OWNED BY public.monedas_07.id;


--
-- Name: monedas_08; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.monedas_08 (
    id integer NOT NULL,
    id_cifrado text NOT NULL,
    estado boolean DEFAULT false,
    fecha_creacion timestamp without time zone DEFAULT CURRENT_TIMESTAMP,
    fecha_minado timestamp without time zone
);


--
-- Name: monedas_08_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.monedas_08_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: monedas_08_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.monedas_08_id_seq OWNED BY public.monedas_08.id;


--
-- Name: monedas_09; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.monedas_09 (
    id integer NOT NULL,
    id_cifrado text NOT NULL,
    estado boolean DEFAULT false,
    fecha_creacion timestamp without time zone DEFAULT CURRENT_TIMESTAMP,
    fecha_minado timestamp without time zone
);


--
-- Name: monedas_09_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.monedas_09_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: monedas_09_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.monedas_09_id_seq OWNED BY public.monedas_09.id;


--
-- Name: monedas_10; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.monedas_10 (
    id integer NOT NULL,
    id_cifrado text NOT NULL,
    estado boolean DEFAULT false,
    fecha_creacion timestamp without time zone DEFAULT CURRENT_TIMESTAMP,
    fecha_minado timestamp without time zone
);


--
-- Name: monedas_10_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.monedas_10_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: monedas_10_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.monedas_10_id_seq OWNED BY public.monedas_10.id;


--
-- Name: monedas_11; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.monedas_11 (
    id integer NOT NULL,
    id_cifrado text NOT NULL,
    estado boolean DEFAULT false,
    fecha_creacion timestamp without time zone DEFAULT CURRENT_TIMESTAMP,
    fecha_minado timestamp without time zone
);


--
-- Name: monedas_11_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.monedas_11_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: monedas_11_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.monedas_11_id_seq OWNED BY public.monedas_11.id;


--
-- Name: monedas_12; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.monedas_12 (
    id integer NOT NULL,
    id_cifrado text NOT NULL,
    estado boolean DEFAULT false,
    fecha_creacion timestamp without time zone DEFAULT CURRENT_TIMESTAMP,
    fecha_minado timestamp without time zone
);


--
-- Name: monedas_12_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.monedas_12_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: monedas_12_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.monedas_12_id_seq OWNED BY public.monedas_12.id;


--
-- Name: monedas_13; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.monedas_13 (
    id integer NOT NULL,
    id_cifrado text NOT NULL,
    estado boolean DEFAULT false,
    fecha_creacion timestamp without time zone DEFAULT CURRENT_TIMESTAMP,
    fecha_minado timestamp without time zone
);


--
-- Name: monedas_13_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.monedas_13_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: monedas_13_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.monedas_13_id_seq OWNED BY public.monedas_13.id;


--
-- Name: monedas_14; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.monedas_14 (
    id integer NOT NULL,
    id_cifrado text NOT NULL,
    estado boolean DEFAULT false,
    fecha_creacion timestamp without time zone DEFAULT CURRENT_TIMESTAMP,
    fecha_minado timestamp without time zone
);


--
-- Name: monedas_14_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.monedas_14_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: monedas_14_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.monedas_14_id_seq OWNED BY public.monedas_14.id;


--
-- Name: monedas_15; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.monedas_15 (
    id integer NOT NULL,
    id_cifrado text NOT NULL,
    estado boolean DEFAULT false,
    fecha_creacion timestamp without time zone DEFAULT CURRENT_TIMESTAMP,
    fecha_minado timestamp without time zone
);


--
-- Name: monedas_15_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.monedas_15_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: monedas_15_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.monedas_15_id_seq OWNED BY public.monedas_15.id;


--
-- Name: monedas_16; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.monedas_16 (
    id integer NOT NULL,
    id_cifrado text NOT NULL,
    estado boolean DEFAULT false,
    fecha_creacion timestamp without time zone DEFAULT CURRENT_TIMESTAMP,
    fecha_minado timestamp without time zone
);


--
-- Name: monedas_16_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.monedas_16_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: monedas_16_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.monedas_16_id_seq OWNED BY public.monedas_16.id;


--
-- Name: monedas_17; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.monedas_17 (
    id integer NOT NULL,
    id_cifrado text NOT NULL,
    estado boolean DEFAULT false,
    fecha_creacion timestamp without time zone DEFAULT CURRENT_TIMESTAMP,
    fecha_minado timestamp without time zone
);


--
-- Name: monedas_17_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.monedas_17_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: monedas_17_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.monedas_17_id_seq OWNED BY public.monedas_17.id;


--
-- Name: monedas_18; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.monedas_18 (
    id integer NOT NULL,
    id_cifrado text NOT NULL,
    estado boolean DEFAULT false,
    fecha_creacion timestamp without time zone DEFAULT CURRENT_TIMESTAMP,
    fecha_minado timestamp without time zone
);


--
-- Name: monedas_18_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.monedas_18_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: monedas_18_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.monedas_18_id_seq OWNED BY public.monedas_18.id;


--
-- Name: monedas_19; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.monedas_19 (
    id integer NOT NULL,
    id_cifrado text NOT NULL,
    estado boolean DEFAULT false,
    fecha_creacion timestamp without time zone DEFAULT CURRENT_TIMESTAMP,
    fecha_minado timestamp without time zone
);


--
-- Name: monedas_19_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.monedas_19_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: monedas_19_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.monedas_19_id_seq OWNED BY public.monedas_19.id;


--
-- Name: monedas_cifradas; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.monedas_cifradas (
    id integer NOT NULL,
    id_cifrado text NOT NULL,
    fecha_creacion timestamp without time zone DEFAULT CURRENT_TIMESTAMP,
    fecha_minado timestamp without time zone,
    porcentaje_minado numeric(10,4) DEFAULT 0.0000
);


--
-- Name: monedas_cifradas_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.monedas_cifradas_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: monedas_cifradas_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.monedas_cifradas_id_seq OWNED BY public.monedas_cifradas.id;


--
-- Name: saldo; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.saldo (
    id integer NOT NULL,
    saldo bigint DEFAULT 0,
    ultima_actualizacion timestamp without time zone DEFAULT CURRENT_TIMESTAMP,
    historial jsonb DEFAULT '[]'::jsonb
);


--
-- Name: saldo_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.saldo_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: saldo_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.saldo_id_seq OWNED BY public.saldo.id;


--
-- Name: ids_originales id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.ids_originales ALTER COLUMN id SET DEFAULT nextval('public.ids_originales_id_seq'::regclass);


--
-- Name: monedas_00 id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.monedas_00 ALTER COLUMN id SET DEFAULT nextval('public.monedas_00_id_seq'::regclass);


--
-- Name: monedas_01 id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.monedas_01 ALTER COLUMN id SET DEFAULT nextval('public.monedas_01_id_seq'::regclass);


--
-- Name: monedas_02 id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.monedas_02 ALTER COLUMN id SET DEFAULT nextval('public.monedas_02_id_seq'::regclass);


--
-- Name: monedas_03 id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.monedas_03 ALTER COLUMN id SET DEFAULT nextval('public.monedas_03_id_seq'::regclass);


--
-- Name: monedas_04 id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.monedas_04 ALTER COLUMN id SET DEFAULT nextval('public.monedas_04_id_seq'::regclass);


--
-- Name: monedas_05 id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.monedas_05 ALTER COLUMN id SET DEFAULT nextval('public.monedas_05_id_seq'::regclass);


--
-- Name: monedas_06 id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.monedas_06 ALTER COLUMN id SET DEFAULT nextval('public.monedas_06_id_seq'::regclass);


--
-- Name: monedas_07 id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.monedas_07 ALTER COLUMN id SET DEFAULT nextval('public.monedas_07_id_seq'::regclass);


--
-- Name: monedas_08 id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.monedas_08 ALTER COLUMN id SET DEFAULT nextval('public.monedas_08_id_seq'::regclass);


--
-- Name: monedas_09 id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.monedas_09 ALTER COLUMN id SET DEFAULT nextval('public.monedas_09_id_seq'::regclass);


--
-- Name: monedas_10 id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.monedas_10 ALTER COLUMN id SET DEFAULT nextval('public.monedas_10_id_seq'::regclass);


--
-- Name: monedas_11 id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.monedas_11 ALTER COLUMN id SET DEFAULT nextval('public.monedas_11_id_seq'::regclass);


--
-- Name: monedas_12 id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.monedas_12 ALTER COLUMN id SET DEFAULT nextval('public.monedas_12_id_seq'::regclass);


--
-- Name: monedas_13 id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.monedas_13 ALTER COLUMN id SET DEFAULT nextval('public.monedas_13_id_seq'::regclass);


--
-- Name: monedas_14 id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.monedas_14 ALTER COLUMN id SET DEFAULT nextval('public.monedas_14_id_seq'::regclass);


--
-- Name: monedas_15 id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.monedas_15 ALTER COLUMN id SET DEFAULT nextval('public.monedas_15_id_seq'::regclass);


--
-- Name: monedas_16 id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.monedas_16 ALTER COLUMN id SET DEFAULT nextval('public.monedas_16_id_seq'::regclass);


--
-- Name: monedas_17 id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.monedas_17 ALTER COLUMN id SET DEFAULT nextval('public.monedas_17_id_seq'::regclass);


--
-- Name: monedas_18 id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.monedas_18 ALTER COLUMN id SET DEFAULT nextval('public.monedas_18_id_seq'::regclass);


--
-- Name: monedas_19 id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.monedas_19 ALTER COLUMN id SET DEFAULT nextval('public.monedas_19_id_seq'::regclass);


--
-- Name: monedas_cifradas id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.monedas_cifradas ALTER COLUMN id SET DEFAULT nextval('public.monedas_cifradas_id_seq'::regclass);


--
-- Name: saldo id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.saldo ALTER COLUMN id SET DEFAULT nextval('public.saldo_id_seq'::regclass);


--
-- Name: ids_originales ids_originales_id_original_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.ids_originales
    ADD CONSTRAINT ids_originales_id_original_key UNIQUE (id_original);


--
-- Name: ids_originales ids_originales_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.ids_originales
    ADD CONSTRAINT ids_originales_pkey PRIMARY KEY (id);


--
-- Name: monedas_00 monedas_00_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.monedas_00
    ADD CONSTRAINT monedas_00_pkey PRIMARY KEY (id);


--
-- Name: monedas_01 monedas_01_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.monedas_01
    ADD CONSTRAINT monedas_01_pkey PRIMARY KEY (id);


--
-- Name: monedas_02 monedas_02_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.monedas_02
    ADD CONSTRAINT monedas_02_pkey PRIMARY KEY (id);


--
-- Name: monedas_03 monedas_03_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.monedas_03
    ADD CONSTRAINT monedas_03_pkey PRIMARY KEY (id);


--
-- Name: monedas_04 monedas_04_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.monedas_04
    ADD CONSTRAINT monedas_04_pkey PRIMARY KEY (id);


--
-- Name: monedas_05 monedas_05_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.monedas_05
    ADD CONSTRAINT monedas_05_pkey PRIMARY KEY (id);


--
-- Name: monedas_06 monedas_06_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.monedas_06
    ADD CONSTRAINT monedas_06_pkey PRIMARY KEY (id);


--
-- Name: monedas_07 monedas_07_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.monedas_07
    ADD CONSTRAINT monedas_07_pkey PRIMARY KEY (id);


--
-- Name: monedas_08 monedas_08_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.monedas_08
    ADD CONSTRAINT monedas_08_pkey PRIMARY KEY (id);


--
-- Name: monedas_09 monedas_09_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.monedas_09
    ADD CONSTRAINT monedas_09_pkey PRIMARY KEY (id);


--
-- Name: monedas_10 monedas_10_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.monedas_10
    ADD CONSTRAINT monedas_10_pkey PRIMARY KEY (id);


--
-- Name: monedas_11 monedas_11_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.monedas_11
    ADD CONSTRAINT monedas_11_pkey PRIMARY KEY (id);


--
-- Name: monedas_12 monedas_12_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.monedas_12
    ADD CONSTRAINT monedas_12_pkey PRIMARY KEY (id);


--
-- Name: monedas_13 monedas_13_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.monedas_13
    ADD CONSTRAINT monedas_13_pkey PRIMARY KEY (id);


--
-- Name: monedas_14 monedas_14_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.monedas_14
    ADD CONSTRAINT monedas_14_pkey PRIMARY KEY (id);


--
-- Name: monedas_15 monedas_15_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.monedas_15
    ADD CONSTRAINT monedas_15_pkey PRIMARY KEY (id);


--
-- Name: monedas_16 monedas_16_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.monedas_16
    ADD CONSTRAINT monedas_16_pkey PRIMARY KEY (id);


--
-- Name: monedas_17 monedas_17_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.monedas_17
    ADD CONSTRAINT monedas_17_pkey PRIMARY KEY (id);


--
-- Name: monedas_18 monedas_18_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.monedas_18
    ADD CONSTRAINT monedas_18_pkey PRIMARY KEY (id);


--
-- Name: monedas_19 monedas_19_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.monedas_19
    ADD CONSTRAINT monedas_19_pkey PRIMARY KEY (id);


--
-- Name: monedas_cifradas monedas_cifradas_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.monedas_cifradas
    ADD CONSTRAINT monedas_cifradas_pkey PRIMARY KEY (id);


--
-- Name: saldo saldo_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.saldo
    ADD CONSTRAINT saldo_pkey PRIMARY KEY (id);


--
-- Name: idx_ids_originales_fecha; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_ids_originales_fecha ON public.ids_originales USING btree (fecha_creacion);


--
-- Name: idx_ids_originales_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_ids_originales_id ON public.ids_originales USING btree (id);


--
-- Name: idx_monedas_00_estado; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_monedas_00_estado ON public.monedas_00 USING btree (estado);


--
-- Name: idx_monedas_00_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_monedas_00_id ON public.monedas_00 USING btree (id);


--
-- Name: idx_monedas_01_estado; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_monedas_01_estado ON public.monedas_01 USING btree (estado);


--
-- Name: idx_monedas_01_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_monedas_01_id ON public.monedas_01 USING btree (id);


--
-- Name: idx_monedas_02_estado; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_monedas_02_estado ON public.monedas_02 USING btree (estado);


--
-- Name: idx_monedas_02_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_monedas_02_id ON public.monedas_02 USING btree (id);


--
-- Name: idx_monedas_03_estado; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_monedas_03_estado ON public.monedas_03 USING btree (estado);


--
-- Name: idx_monedas_03_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_monedas_03_id ON public.monedas_03 USING btree (id);


--
-- Name: idx_monedas_04_estado; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_monedas_04_estado ON public.monedas_04 USING btree (estado);


--
-- Name: idx_monedas_04_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_monedas_04_id ON public.monedas_04 USING btree (id);


--
-- Name: idx_monedas_05_estado; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_monedas_05_estado ON public.monedas_05 USING btree (estado);


--
-- Name: idx_monedas_05_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_monedas_05_id ON public.monedas_05 USING btree (id);


--
-- Name: idx_monedas_06_estado; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_monedas_06_estado ON public.monedas_06 USING btree (estado);


--
-- Name: idx_monedas_06_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_monedas_06_id ON public.monedas_06 USING btree (id);


--
-- Name: idx_monedas_07_estado; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_monedas_07_estado ON public.monedas_07 USING btree (estado);


--
-- Name: idx_monedas_07_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_monedas_07_id ON public.monedas_07 USING btree (id);


--
-- Name: idx_monedas_08_estado; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_monedas_08_estado ON public.monedas_08 USING btree (estado);


--
-- Name: idx_monedas_08_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_monedas_08_id ON public.monedas_08 USING btree (id);


--
-- Name: idx_monedas_09_estado; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_monedas_09_estado ON public.monedas_09 USING btree (estado);


--
-- Name: idx_monedas_09_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_monedas_09_id ON public.monedas_09 USING btree (id);


--
-- Name: idx_monedas_10_estado; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_monedas_10_estado ON public.monedas_10 USING btree (estado);


--
-- Name: idx_monedas_10_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_monedas_10_id ON public.monedas_10 USING btree (id);


--
-- Name: idx_monedas_11_estado; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_monedas_11_estado ON public.monedas_11 USING btree (estado);


--
-- Name: idx_monedas_11_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_monedas_11_id ON public.monedas_11 USING btree (id);


--
-- Name: idx_monedas_12_estado; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_monedas_12_estado ON public.monedas_12 USING btree (estado);


--
-- Name: idx_monedas_12_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_monedas_12_id ON public.monedas_12 USING btree (id);


--
-- Name: idx_monedas_13_estado; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_monedas_13_estado ON public.monedas_13 USING btree (estado);


--
-- Name: idx_monedas_13_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_monedas_13_id ON public.monedas_13 USING btree (id);


--
-- Name: idx_monedas_14_estado; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_monedas_14_estado ON public.monedas_14 USING btree (estado);


--
-- Name: idx_monedas_14_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_monedas_14_id ON public.monedas_14 USING btree (id);


--
-- Name: idx_monedas_15_estado; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_monedas_15_estado ON public.monedas_15 USING btree (estado);


--
-- Name: idx_monedas_15_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_monedas_15_id ON public.monedas_15 USING btree (id);


--
-- Name: idx_monedas_16_estado; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_monedas_16_estado ON public.monedas_16 USING btree (estado);


--
-- Name: idx_monedas_16_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_monedas_16_id ON public.monedas_16 USING btree (id);


--
-- Name: idx_monedas_17_estado; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_monedas_17_estado ON public.monedas_17 USING btree (estado);


--
-- Name: idx_monedas_17_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_monedas_17_id ON public.monedas_17 USING btree (id);


--
-- Name: idx_monedas_18_estado; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_monedas_18_estado ON public.monedas_18 USING btree (estado);


--
-- Name: idx_monedas_18_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_monedas_18_id ON public.monedas_18 USING btree (id);


--
-- Name: idx_monedas_19_estado; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_monedas_19_estado ON public.monedas_19 USING btree (estado);


--
-- Name: idx_monedas_19_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_monedas_19_id ON public.monedas_19 USING btree (id);


--
-- Name: idx_monedas_fecha_creacion; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_monedas_fecha_creacion ON public.monedas_cifradas USING btree (fecha_creacion);


--
-- Name: idx_monedas_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_monedas_id ON public.monedas_cifradas USING btree (id);


--
-- Name: idx_monedas_porcentaje; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_monedas_porcentaje ON public.monedas_cifradas USING btree (porcentaje_minado);


--
-- PostgreSQL database dump complete
--

\unrestrict zaASbSF0isme7iuaDeYQCMNOhgum1BqGvj7hCUI9rmhFvNiiCH1aSXB3je7pKHN

