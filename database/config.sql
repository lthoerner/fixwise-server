DROP SCHEMA IF EXISTS test CASCADE;

CREATE SCHEMA test;

CREATE TABLE test.inventory (
    sku             serial PRIMARY KEY,
    display_name    text NOT NULL,
    count           integer NOT NULL,
    cost            numeric(1000, 2) NOT NULL,
    price           numeric(1000, 2) NOT NULL
);

CREATE TABLE test.customers (
    id              serial PRIMARY KEY,
    name            text NOT NULL,
    email           text NOT NULL,
    phone           text NOT NULL,
    address         text
);

CREATE VIEW test.inventory_view AS
    SELECT sku, display_name, count, cost, price FROM test.inventory ORDER BY sku ASC;

CREATE VIEW test.customers_view AS
    SELECT id, name, email, phone, address FROM test.customers ORDER BY id ASC;
