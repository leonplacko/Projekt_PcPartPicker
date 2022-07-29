-- Your SQL goes here
CREATE TABLE GPU(
    id VARCHAR(36) DEFAULT uuid_generate_v4() NOT NULL,
    name VARCHAR(255) NOT NULL,
    manufacturer VARCHAR(50) NOT NULL,
    memory INTEGER NOT NULL,
    memory_type VARCHAR NOT NULL,
    tdp INTEGER NOT NULL,
    price REAL NOT NULL,
    PRIMARY KEY(id)
);