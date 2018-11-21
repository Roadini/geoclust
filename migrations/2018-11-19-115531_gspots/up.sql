-- Your SQL goes here
CREATE TABLE gspots(
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    address VARCHAR NOT NULL,
    google_place_id VARCHAR NOT NULL,
    lat REAL NOT NULL,
    lng REAL NOT NULL,
    primary_type VARCHAR NOT NULL,
    secondary_type VARCHAR NOT NULL
)
