DEFINE TABLE inventory SCHEMAFULL;
DEFINE FIELD display_name ON inventory TYPE string;
DEFINE FIELD sku ON inventory TYPE int;
DEFINE FIELD count ON inventory TYPE int;
DEFINE FIELD cost ON inventory TYPE decimal;
DEFINE FIELD price ON inventory TYPE decimal;

CREATE inventory SET display_name = 'iPhone 15', sku = 000001, count = 4, cost = 700.0, price = 950.50;
CREATE inventory SET display_name = 'Samsung S24', sku = 000002, count = 9, cost = 650.0, price = 774.99;
CREATE inventory SET display_name = 'Pixel 8', sku = 000003, count = 2, cost = 500.0, price = 595.0;
CREATE inventory SET display_name = 'Sony Xperia', sku = 000004, count = 1, cost = 1050.0, price = 1299.99;
CREATE inventory SET display_name = 'LG G9', sku = 000005, count = 3, cost = 250.0, price = 355.0;
CREATE inventory SET display_name = 'Motorola Edge', sku = 000006, count = 15, cost = 199.99, price = 214.99;
