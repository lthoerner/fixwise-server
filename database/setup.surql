DEFINE TABLE schema SCHEMAFULL;
DEFINE FIELD table_fields ON schema TYPE array<object>;
DEFINE FIELD table_fields.* ON schema TYPE object;
DEFINE FIELD table_fields.*.true_name ON schema TYPE string;
DEFINE FIELD table_fields.*.display_name ON schema TYPE string;
DEFINE FIELD table_fields.*.search_weight ON schema TYPE int;
DEFINE FIELD table_fields.*.data_type ON schema TYPE string;
DEFINE FIELD table_fields.*.formatting ON schema TYPE option<object>;
DEFINE FIELD table_fields.*.formatting.prefix ON schema TYPE option<string>;
DEFINE FIELD table_fields.*.formatting.suffix ON schema TYPE option<string>;
DEFINE FIELD table_fields.*.formatting.pad_length ON schema TYPE option<int>;

DEFINE TABLE inventory SCHEMAFULL;
DEFINE FIELD sku ON inventory TYPE int;
DEFINE FIELD display_name ON inventory TYPE string;
DEFINE FIELD count ON inventory TYPE int;
DEFINE FIELD cost ON inventory TYPE decimal;
DEFINE FIELD price ON inventory TYPE decimal;

CREATE schema:inventory SET table_fields = [
  { "true_name": "sku", "display_name": "SKU", "search_weight": 1, "data_type": "int", "formatting": { "prefix": "#", "pad_length": 7 } },
  { "true_name": "display_name", "display_name": "Name", "search_weight": 1, "data_type": "string" },
  { "true_name": "count", "display_name": "Count", "search_weight": 1, "data_type": "int" },
  { "true_name": "cost", "display_name": "Cost", "search_weight": 1, "data_type": "decimal", "formatting": { "prefix": "$" } },
  { "true_name": "price", "display_name": "Price", "search_weight": 1, "data_type": "decimal", "formatting": { "prefix": "$" } }
];
