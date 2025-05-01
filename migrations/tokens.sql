CREATE TABLE IF NOT EXISTS public.tokens (
	seed varchar(16) NOT NULL,
	bits int4 NOT NULL,
	stamp varchar(256) NOT NULL,
	CONSTRAINT tokens_pk PRIMARY KEY (seed)
);