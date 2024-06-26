DROP SCHEMA IF EXISTS main CASCADE;

DROP TYPE IF EXISTS ticket_status;

CREATE SCHEMA main;

CREATE TYPE ticket_status AS ENUM (
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
    vendor integer references main.vendors (id) NOT NULL,
    manufacturer integer references main.part_manufacturers (id),
    category integer references main.part_categories (id) NOT NULL,
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
    model integer references main.device_models (id) NOT NULL,
    owner integer references main.customers (id)
);

CREATE TABLE main.tickets (
    id serial PRIMARY KEY,
    status ticket_status NOT NULL,
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

CREATE VIEW main.vendors_view AS
SELECT
    id,
    display_name
FROM
    main.vendors
ORDER BY
    id ASC;

CREATE VIEW main.customers_view AS
SELECT
    id,
    name,
    email_address,
    phone_number,
    street_address
FROM
    main.customers
ORDER BY
    id ASC;

CREATE VIEW main.device_models_view AS
SELECT
    model.id,
    model.display_name,
    manufacturer.display_name AS manufacturer,
    category.display_name AS category
FROM
    main.device_models model
    LEFT JOIN main.device_manufacturers manufacturer ON model.manufacturer = manufacturer.id
    LEFT JOIN main.device_categories category ON model.category = category.id
ORDER BY
    id ASC;

CREATE VIEW main.devices_view AS
SELECT
    device.id,
    model.display_name AS model,
    customer.name AS owner
FROM
    main.devices device
    LEFT JOIN main.device_models model ON device.model = model.id
    LEFT JOIN main.customers customer ON device.owner = customer.id
ORDER BY
    id ASC;

CREATE VIEW main.parts_view AS
SELECT
    part.id,
    part.display_name,
    vendor.display_name AS vendor,
    manufacturer.display_name AS manufacturer,
    category.display_name AS category,
    part.cost,
    part.price
FROM
    main.parts part
    LEFT JOIN main.vendors vendor ON part.vendor = vendor.id
    LEFT JOIN main.part_manufacturers manufacturer ON part.manufacturer = manufacturer.id
    LEFT JOIN main.part_categories category ON part.category = category.id
ORDER BY
    id ASC;

CREATE VIEW main.tickets_view AS
SELECT
    ticket.id,
    ticket.status,
    customer.name AS customer,
    ticket.invoice_total - ticket.payment_total AS balance,
    ticket.created_at,
    ticket.updated_at
FROM
    main.tickets ticket
    LEFT JOIN main.customers customer ON ticket.customer = customer.id
ORDER BY
    id ASC;