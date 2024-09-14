DROP SCHEMA IF EXISTS main CASCADE;

DROP TYPE IF EXISTS ticket_status;
DROP TYPE IF EXISTS payment_type;
DROP TYPE IF EXISTS item_type;

CREATE SCHEMA IF NOT EXISTS persistent;

CREATE SCHEMA main;

CREATE TYPE ticket_status AS ENUM (
    'new',
    'waiting_for_parts',
    'waiting_for_customer',
    'in_repair',
    'ready_for_pickup',
    'closed'
);

CREATE TYPE payment_type AS ENUM ('card', 'cash');

CREATE TYPE item_type AS ENUM ('product', 'service');

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
    cost numeric(1000, 2) NOT NULL DEFAULT 0,
    price numeric(1000, 2) NOT NULL DEFAULT 0
);

-- This table is a stub to be expanded upon later
CREATE TABLE main.products (
    sku serial PRIMARY KEY,
    display_name text NOT NULL
);

CREATE TABLE main.product_prices (
    id serial PRIMARY KEY,
    product integer references main.products (sku) NOT NULL,
    cost numeric(1000, 2) NOT NULL DEFAULT 0,
    price numeric(1000, 2) NOT NULL DEFAULT 0,
    time_set timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE main.service_types (
    id serial PRIMARY KEY,
    display_name text NOT NULL
);

CREATE TABLE main.services (
    id serial PRIMARY KEY,
    -- "Other" device and type must be actual static records
    type integer references main.service_types (id) NOT NULL,
    device integer references main.device_models (id) NOT NULL
);

CREATE TABLE main.service_prices (
    id serial PRIMARY KEY,
    service integer references main.services (id) NOT NULL,
    base_fee numeric(1000, 2) NOT NULL DEFAULT 0,
    labor_fee numeric(1000, 2) NOT NULL DEFAULT 0,
    time_set timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE main.items (
    id serial PRIMARY KEY,
    product_or_service integer NOT NULL,
    type item_type NOT NULL,
    UNIQUE (product_or_service, type)
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

CREATE TABLE main.invoices (
    id serial PRIMARY KEY,
    created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE main.invoice_items (
    invoice integer references main.invoices (id) NOT NULL,
    item integer references main.items (id) NOT NULL,
    PRIMARY KEY (invoice, item)
);

CREATE TABLE main.invoice_payments (
    id serial PRIMARY KEY,
    invoice integer references main.invoices (id) NOT NULL,
    amount numeric(1000, 2) NOT NULL,
    type payment_type NOT NULL,
    timestamp timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE main.tickets (
    id serial PRIMARY KEY,
    status ticket_status NOT NULL DEFAULT 'new',
    customer integer references main.customers (id),
    invoice integer references main.invoices (id),
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
    service integer references main.services (id),
    diagnostic text,
    PRIMARY KEY (ticket, device)
);

CREATE TABLE main.bundled_parts (
    ticket integer references main.tickets (id),
    device integer references main.devices (id),
    part integer references main.parts (id),
    PRIMARY KEY (ticket, device, part),
    FOREIGN KEY (ticket, device) references main.ticket_devices (ticket, device)
);

CREATE TABLE IF NOT EXISTS persistent.type_allocation_codes (
    tac serial PRIMARY KEY,
    manufacturer text NOT NULL,
    model text NOT NULL
);

CREATE FUNCTION main.get_product_price_at_time(product_id integer, point_in_time timestamp)
RETURNS numeric AS $$
BEGIN
    RETURN (
        SELECT
            price
        FROM
            main.product_prices
        WHERE
            product = product_id
            AND time_set <= point_in_time
        ORDER BY
            time_set DESC
        LIMIT
            1
    );
END;
$$ LANGUAGE plpgsql;

CREATE FUNCTION main.get_service_price_at_time(service_id integer, point_in_time timestamp)
RETURNS numeric AS $$
BEGIN
    RETURN (
        SELECT
            base_fee + labor_fee
        FROM
            main.service_prices
        WHERE
            service = service_id
            AND time_set <= point_in_time
        ORDER BY
            time_set DESC
        LIMIT
            1
    );
END;
$$ LANGUAGE plpgsql;

CREATE FUNCTION main.get_item_price_at_time(item_id integer, point_in_time timestamp)
RETURNS numeric AS $$
DECLARE
    item_type item_type;
    product_or_service_id integer;
    price numeric;
BEGIN
    SELECT
        type,
        product_or_service INTO item_type,
        product_or_service_id
    FROM
        main.items
    WHERE
        id = item_id;

    IF item_type = 'product' THEN
        price := main.get_product_price_at_time(product_or_service_id, point_in_time);
    ELSIF item_type = 'service' THEN
        price := main.get_service_price_at_time(product_or_service_id, point_in_time);
    END IF;

    RETURN COALESCE(price, '0');
END;
$$ LANGUAGE plpgsql;

CREATE FUNCTION main.get_invoice_total(invoice_id integer)
RETURNS numeric AS $$
DECLARE
    creation_date timestamp;
BEGIN
    SELECT
        created_at INTO creation_date
    FROM
        main.invoices
    WHERE
        id = invoice_id;

    RETURN (
        SELECT
            COALESCE(SUM(main.get_item_price_at_time(item_id, creation_date)), '0')
        FROM
            (
                SELECT
                    item AS item_id
                FROM
                    main.invoice_items
                WHERE
                    invoice = invoice_id
            )
    );
END;
$$ LANGUAGE plpgsql;

CREATE FUNCTION main.get_payment_total(invoice_id integer)
RETURNS numeric AS $$
BEGIN
    RETURN (
        SELECT
            COALESCE(SUM(amount), '0')
        FROM
            main.invoice_payments
        WHERE
            invoice = invoice_id
    );
END;
$$ LANGUAGE plpgsql;

CREATE FUNCTION main.get_invoice_balance(invoice_id integer)
RETURNS numeric AS $$
BEGIN
    RETURN (
        main.get_invoice_total(invoice_id) - main.get_payment_total(invoice_id)
    );
END;
$$ LANGUAGE plpgsql;

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
    main.get_invoice_balance(ticket.invoice) AS balance,
    ticket.created_at,
    ticket.updated_at
FROM
    main.tickets ticket
    LEFT JOIN main.customers customer ON ticket.customer = customer.id
ORDER BY
    id ASC;

CREATE FUNCTION main.insert_product_as_item()
RETURNS TRIGGER AS $$
BEGIN
    INSERT INTO
        main.items (product_or_service, type)
    VALUES
        (NEW.sku, 'product');

    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE FUNCTION main.insert_service_as_item()
RETURNS TRIGGER AS $$
BEGIN
    INSERT INTO
        main.items (product_or_service, type)
    VALUES
        (NEW.id, 'service');

    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER product_item_collector
AFTER
INSERT
    ON main.products FOR EACH ROW EXECUTE FUNCTION main.insert_product_as_item();

CREATE TRIGGER service_item_collector
AFTER
INSERT
    ON main.services FOR EACH ROW EXECUTE FUNCTION main.insert_service_as_item();