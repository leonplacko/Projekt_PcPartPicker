-- Your SQL goes here
CREATE TABLE ManuFacturer(
    id VARCHAR NOT NULL DEFAULT uuid_generate_v4(),
    name VARCHAR(50) NOT NULL,
    PRIMARY KEY(id)
);