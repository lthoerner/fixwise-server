DROP SCHEMA IF EXISTS main CASCADE;

CREATE SCHEMA main;

CREATE TYPE main.ticket_status AS ENUM (
    'new',
    'waiting_for_parts',
    'waiting_for_customer',
    'in_repair',
    'ready_for_pickup',
    'closed'
);

CREATE TABLE main.vendors (
    id serial PRIMARY KEY,
    display_name text NOT NULL,
    email_address text,
    phone_number text,
    street_address text
);

CREATE TABLE main.device_manufacturers (
    id serial PRIMARY KEY,
    display_name text NOT NULL
);

CREATE TABLE main.part_manufacturers (
    id serial PRIMARY KEY,
    display_name text NOT NULL
);

CREATE TABLE main.device_categories (
    id serial PRIMARY KEY,
    display_name text NOT NULL
);

CREATE TABLE main.part_categories (
    id serial PRIMARY KEY,
    display_name text NOT NULL
);

CREATE TABLE main.device_models (
    id serial PRIMARY KEY,
    display_name text NOT NULL,
    primary_model_identifiers text [] NOT NULL DEFAULT '{}',
    secondary_model_identifiers text [] NOT NULL DEFAULT '{}',
    manufacturer integer references main.device_manufacturers (id) NOT NULL,
    category integer references main.device_categories (id) NOT NULL
);

CREATE TABLE main.parts (
    id serial PRIMARY KEY,
    display_name text NOT NULL,
    vendor integer references main.vendors (id),
    manufacturer integer references main.part_manufacturers (id),
    category integer references main.part_categories (id),
    cost numeric(1000, 2),
    price numeric(1000, 2)
);

CREATE TABLE main.customers (
    id serial PRIMARY KEY,
    name text NOT NULL,
    email_address text,
    phone_number text,
    street_address text
);

CREATE TABLE main.devices (
    id serial PRIMARY KEY,
    model integer references main.device_models (id),
    owner integer references main.customers (id)
);

CREATE TABLE main.tickets (
    id serial PRIMARY KEY,
    status main.ticket_status NOT NULL,
    customer integer references main.customers (id),
    invoice_total numeric(1000, 2) NOT NULL,
    payment_total numeric(1000, 2) NOT NULL,
    description text,
    notes text [] NOT NULL DEFAULT '{}',
    created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE main.compatible_parts (
    device integer references main.device_models (id),
    part integer references main.parts (id),
    PRIMARY KEY (device, part)
);

CREATE TABLE main.ticket_devices (
    ticket integer references main.tickets (id),
    device integer references main.devices (id),
    diagnostic text,
    labor_fee numeric(1000, 2),
    PRIMARY KEY (ticket, device)
);

CREATE TABLE main.bundled_parts (
    ticket integer references main.tickets (id),
    device integer references main.devices (id),
    part integer references main.parts (id),
    PRIMARY KEY (ticket, device, part),
    FOREIGN KEY (ticket, device) references main.ticket_devices (ticket, device)
);