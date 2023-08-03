CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS penguins
(
    id uuid DEFAULT uuid_generate_v1() NOT NULL CONSTRAINT penguins_pkey PRIMARY KEY,
    penguin_name text NOT NULL,
    age smallint NOT NULL,
    created_at timestamp with time zone default CURRENT_TIMESTAMP,
    updated_at timestamp with time zone
);