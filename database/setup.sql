DROP TABLE IF EXISTS inventory CASCADE;

CREATE TABLE inventory (
    sku             serial PRIMARY KEY,
    display_name    text NOT NULL,
    count           integer NOT NULL,
    cost            numeric(1000, 2) NOT NULL,
    price           numeric(1000, 2) NOT NULL
);
