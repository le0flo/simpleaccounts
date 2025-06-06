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
	method varchar(4) NOT NULL,
	secret text NULL,
	CONSTRAINT users_pk PRIMARY KEY (identifier)
);

-- public.wallets
CREATE TABLE IF NOT EXISTS public.wallets (
    identifier varchar(20) NOT NULL,
	balance int4 DEFAULT 0 NOT NULL,
	CONSTRAINT wallets_pk PRIMARY KEY (identifier)
);

-- public.wallets foreign keys
ALTER TABLE public.wallets ADD CONSTRAINT wallets_users_fk FOREIGN KEY (identifier) REFERENCES public.users(identifier);
