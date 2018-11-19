
CREATE TABLE visits (
    id SERIAL PRIMARY KEY,
    list_id INTEGER NOT NULL,
    internal_id_place INTEGER NOT NULL,
    review VARCHAR NOT NULL
)
