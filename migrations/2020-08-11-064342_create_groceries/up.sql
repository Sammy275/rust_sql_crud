CREATE TABLE groceries (
    id SERIAL PRIMARY KEY,
    item_name VARCHAR NOT NULL,
    quantity INT NOT NULL,
    price INT NOT NULL
)