DROP SCHEMA IF EXISTS test CASCADE;

CREATE SCHEMA test;

CREATE TABLE
    test.inventory (
        sku serial PRIMARY KEY,
        name text NOT NULL,
        count integer NOT NULL,
        cost numeric(1000, 2) NOT NULL,
        price numeric(1000, 2) NOT NULL
    );

CREATE TABLE
    test.customers (
        id serial PRIMARY KEY,
        name text NOT NULL,
        email text NOT NULL,
        phone text NOT NULL,
        address text
    );

CREATE TABLE
    test.tickets (
        id serial PRIMARY KEY,
        customer_id integer references test.customers (id) NOT NULL,
        device text NOT NULL,
        diagnostic text NOT NULL,
        invoice_amount numeric(1000, 2) NOT NULL DEFAULT 0,
        payment_amount numeric(1000, 2) NOT NULL DEFAULT 0,
        created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
        updated_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP
    );

CREATE VIEW
    test.inventory_view AS
SELECT
    sku,
    name,
    count,
    cost,
    price
FROM
    test.inventory
ORDER BY
    sku ASC;

CREATE VIEW
    test.customers_view AS
SELECT
    id,
    name,
    email,
    phone,
    address
FROM
    test.customers
ORDER BY
    id ASC;

CREATE VIEW
    test.tickets_view AS
SELECT
    ticket.id,
    customer.name AS customer_name,
    ticket.device,
    ticket.invoice_amount - ticket.payment_amount AS balance,
    ticket.created_at,
    ticket.updated_at
FROM
    test.tickets ticket
    JOIN test.customers customer ON ticket.customer_id = customer.id
ORDER BY
    id ASC;