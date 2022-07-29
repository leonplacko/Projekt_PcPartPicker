-- Your SQL goes here
CREATE TABLE Power_unit(
    id VARCHAR(36) DEFAULT uuid_generate_v4() NOT NULL,
    name VARCHAR(255) NOT NULL,
    manufacturer VARCHAR(50) NOT NULL,
    power INTEGER NOT NULL,
    price REAL NOT NULL,
    PRIMARY KEY(id)
);