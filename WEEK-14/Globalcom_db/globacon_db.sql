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
-- Name: customertable; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.customertable (
    cid integer NOT NULL,
    cname text NOT NULL,
    cage integer NOT NULL,
    cemail text NOT NULL,
    cmobile character varying(15) NOT NULL,
    eid integer NOT NULL,
    dataid integer NOT NULL
);


ALTER TABLE public.customertable OWNER TO postgres;

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
-- Name: department; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.department (
    dno integer NOT NULL,
    did integer NOT NULL,
    dname text NOT NULL,
    dlocation text NOT NULL,
    pno integer NOT NULL
);


ALTER TABLE public.department OWNER TO postgres;

--
-- Name: project; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.project (
    pno integer NOT NULL,
    pname text NOT NULL,
    pduration text NOT NULL,
    pid integer NOT NULL
);


ALTER TABLE public.project OWNER TO postgres;

--
-- Name: staff; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.staff (
    staff_id integer NOT NULL,
    staff_name text NOT NULL,
    eno integer NOT NULL,
    staff_sal real,
    age integer NOT NULL,
    phone character varying(15) NOT NULL
);


ALTER TABLE public.staff OWNER TO postgres;

--
-- Data for Name: customertable; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.customertable (cid, cname, cage, cemail, cmobile, eid, dataid) FROM stdin;
110	Musta Karim	35	m_karim@gmail.com	8055089112	102	5
111	Lilian Jaiya	43	l_jaiye@gmail.com	8055185341	100	3
112	Arthur Musa	50	a_musa@gmail.com	7055282813	107	10
113	Philip Akenjo	41	p_akenjo@gmail.com	9052356772	100	2
114	Marylene Mapa	33	m_mapa@gmail.com	8053333551	120	5
115	Oghenero Agor	50	o_agor@gmail.com	7055566774	117	11
116	Adams Bree	33	a_bree@gmail.com	8056765424	102	1
117	Okafor Mathias	45	o_mathias@gmail.com	8056763367	120	10
118	Samson Adeleke	65	s_adeleke@gmail.com	7056774423	117	11
119	Lawal Tamire	35	l_tamire@gmail.com	9052111101	107	5
120	James Job	44	j_job@gmail.com	8059693919	100	8
121	Matthew Jakande	21	m_jakande@gmail.com	7051232144	120	2
122	Jimila Adegboye	20	j_adegboye@gmail.com	8054921923	107	5
\.


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
-- Data for Name: department; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.department (dno, did, dname, dlocation, pno) FROM stdin;
1	108	Administration	Ikeja	44
2	101	Account	Egbeda	11
3	100	Packaging	Ajah	44
4	120	Research	V.I	33
5	97	Account	Magodo	22
6	122	Operations	Mile 2	44
7	107	Packaging	Ketu	55
\.


--
-- Data for Name: project; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.project (pno, pname, pduration, pid) FROM stdin;
11	A	9 Months	102
22	B	14 Months	97
33	C	16 Months	120
44	D	25 Months	108
55	E	9 Months	107
\.


--
-- Data for Name: staff; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.staff (staff_id, staff_name, eno, staff_sal, age, phone) FROM stdin;
101	Alade Joy	2	250000	33	8023089832
100	Mustapha Ali	3	175000	32	8063285831
107	Alokwe Martin	7	380000	48	7090082812
97	Dankade Aminat	5	550000	40	9023688832
108	Josaih Joshua	1	120000	30	8053189131
102	Makinde Mary	2	450000	55	9023487830
120	Adeleke Jane	4	200000	38	7061045862
122	Osahon Mark	6	320000	44	8022289842
117	Suleman Ajayi	3	800000	50	7030089811
104	Kuti Lawal	1	750000	35	9145689842
\.


--
-- Name: customertable customertable_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.customertable
    ADD CONSTRAINT customertable_pkey PRIMARY KEY (cid);


--
-- Name: dataplantable dataplantable_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.dataplantable
    ADD CONSTRAINT dataplantable_pkey PRIMARY KEY (dataid);


--
-- Name: department department_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.department
    ADD CONSTRAINT department_pkey PRIMARY KEY (dno);


--
-- Name: staff employees_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.staff
    ADD CONSTRAINT employees_pkey PRIMARY KEY (staff_id);


--
-- Name: project project_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.project
    ADD CONSTRAINT project_pkey PRIMARY KEY (pno);


--
-- PostgreSQL database dump complete
--

