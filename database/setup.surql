DEFINE TABLE inventory SCHEMAFULL;
DEFINE FIELD sku ON inventory TYPE int;
DEFINE FIELD display_name ON inventory TYPE string;

CREATE inventory SET sku = 000001, display_name = 'iPhone 15';
CREATE inventory SET sku = 000002, display_name = 'Samsung S24';
CREATE inventory SET sku = 000003, display_name = 'Pixel 8';
