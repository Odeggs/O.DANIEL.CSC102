--
-- PostgreSQL database dump
--

-- Dumped from database version 16.1
-- Dumped by pg_dump version 16.1

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
-- Name: dataplantable; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.dataplantable (
    dataid integer NOT NULL,
    datasize text NOT NULL,
    dataduration integer NOT NULL,
    dataprice bigint NOT NULL
);


ALTER TABLE public.dataplantable OWNER TO postgres;

--
-- Data for Name: dataplantable; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.dataplantable (dataid, datasize, dataduration, dataprice) FROM stdin;
1	350MB	2	200
2	1.8GB	14	500
3	3.9GB	30	1000
4	7.5GB	30	1500
5	9.2GB	30	2000
6	10.8GB	30	2500
7	14GB	30	30000
8	18GB	30	4000
9	24GB	30	5000
10	29.9GB	30	8000
11	50GB	30	10000
\.


--
-- Name: dataplantable dataplantable_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.dataplantable
    ADD CONSTRAINT dataplantable_pkey PRIMARY KEY (dataid);


--
-- PostgreSQL database dump complete
--

