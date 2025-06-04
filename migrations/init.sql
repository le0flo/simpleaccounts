-- Database
\c simpleaccounts

-- Tokens
CREATE TABLE IF NOT EXISTS public.tokens (
	seed varchar(16) NOT NULL,
	bits int4 NOT NULL,
	stamp varchar(256) NOT NULL,
	CONSTRAINT tokens_pk PRIMARY KEY (seed)
);

-- Users
CREATE TABLE IF NOT EXISTS public.users (
	identifier varchar(20) NOT NULL,
	balance int4 DEFAULT 0 NOT NULL,
	CONSTRAINT users_pk PRIMARY KEY (identifier)
);
