-- Database
\c simpleaccounts

-- public.tokens
CREATE TABLE IF NOT EXISTS public.tokens (
	seed varchar(16) NOT NULL,
	bits int4 NOT NULL,
	stamp varchar(256) NOT NULL,
	CONSTRAINT tokens_pk PRIMARY KEY (seed)
);

-- public.users
CREATE TABLE IF NOT EXISTS public.users (
	identifier varchar(20) NOT NULL,
	balance int4 DEFAULT 0 NOT NULL,
	CONSTRAINT users_pk PRIMARY KEY (identifier)
);

-- public.auth
CREATE TABLE IF NOT EXISTS public.auth (
	identifier varchar(20) NOT NULL,
	method varchar(4) NOT NULL,
	secret text NULL,
	CONSTRAINT auth_pk PRIMARY KEY (identifier)
);

-- public.auth, foreign keys
ALTER TABLE public.auth ADD CONSTRAINT auth_users_fk FOREIGN KEY (identifier) REFERENCES public.users(identifier);
