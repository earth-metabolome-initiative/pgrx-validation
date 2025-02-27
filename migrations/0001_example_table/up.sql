CREATE TABLE IF NOT EXISTS price (
    value INT CHECK (strictly_positive(value))
);