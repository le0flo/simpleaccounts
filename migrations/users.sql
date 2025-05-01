CREATE TABLE IF NOT EXISTS public.users (
	identifier varchar(20) NOT NULL,
	balance int4 DEFAULT 0 NOT NULL,
	CONSTRAINT users_pk PRIMARY KEY (identifier)
);
