--
-- PostgreSQL database dump
--

\restrict jJlTlhv2OpTX6tVfzKSmrIsTdmVSQxjswb17SM2u3gkISkF0lMzCopiNLlNPDKX

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
-- Name: ids_originales; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.ids_originales (
    id integer NOT NULL,
    id_original text NOT NULL,
    fecha_creacion timestamp without time zone DEFAULT CURRENT_TIMESTAMP
);


ALTER TABLE public.ids_originales OWNER TO postgres;

--
-- Name: ids_originales_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.ids_originales_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.ids_originales_id_seq OWNER TO postgres;

--
-- Name: ids_originales_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.ids_originales_id_seq OWNED BY public.ids_originales.id;


--
-- Name: monedas_cifradas; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.monedas_cifradas (
    id integer NOT NULL,
    id_cifrado text NOT NULL,
    estado boolean DEFAULT false,
    fecha_creacion timestamp without time zone DEFAULT CURRENT_TIMESTAMP,
    fecha_minado timestamp without time zone
);


ALTER TABLE public.monedas_cifradas OWNER TO postgres;

--
-- Name: monedas_cifradas_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.monedas_cifradas_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.monedas_cifradas_id_seq OWNER TO postgres;

--
-- Name: monedas_cifradas_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.monedas_cifradas_id_seq OWNED BY public.monedas_cifradas.id;


--
-- Name: saldo; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.saldo (
    id integer NOT NULL,
    saldo bigint DEFAULT 0,
    ultima_actualizacion timestamp without time zone DEFAULT CURRENT_TIMESTAMP,
    historial jsonb DEFAULT '[]'::jsonb
);


ALTER TABLE public.saldo OWNER TO postgres;

--
-- Name: saldo_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.saldo_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.saldo_id_seq OWNER TO postgres;

--
-- Name: saldo_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.saldo_id_seq OWNED BY public.saldo.id;


--
-- Name: ids_originales id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.ids_originales ALTER COLUMN id SET DEFAULT nextval('public.ids_originales_id_seq'::regclass);


--
-- Name: monedas_cifradas id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.monedas_cifradas ALTER COLUMN id SET DEFAULT nextval('public.monedas_cifradas_id_seq'::regclass);


--
-- Name: saldo id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.saldo ALTER COLUMN id SET DEFAULT nextval('public.saldo_id_seq'::regclass);


--
-- Name: ids_originales ids_originales_id_original_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.ids_originales
    ADD CONSTRAINT ids_originales_id_original_key UNIQUE (id_original);


--
-- Name: ids_originales ids_originales_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.ids_originales
    ADD CONSTRAINT ids_originales_pkey PRIMARY KEY (id);


--
-- Name: monedas_cifradas monedas_cifradas_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.monedas_cifradas
    ADD CONSTRAINT monedas_cifradas_pkey PRIMARY KEY (id);


--
-- Name: saldo saldo_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.saldo
    ADD CONSTRAINT saldo_pkey PRIMARY KEY (id);


--
-- Name: idx_ids_originales_fecha; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_ids_originales_fecha ON public.ids_originales USING btree (fecha_creacion);


--
-- Name: idx_monedas_estado; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_monedas_estado ON public.monedas_cifradas USING btree (estado);


--
-- Name: idx_monedas_fecha_creacion; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_monedas_fecha_creacion ON public.monedas_cifradas USING btree (fecha_creacion);


--
-- Name: idx_monedas_id; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_monedas_id ON public.monedas_cifradas USING btree (id);


--
-- PostgreSQL database dump complete
--

\unrestrict jJlTlhv2OpTX6tVfzKSmrIsTdmVSQxjswb17SM2u3gkISkF0lMzCopiNLlNPDKX

