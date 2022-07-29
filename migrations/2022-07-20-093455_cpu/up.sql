-- Your SQL goes here
CREATE TABLE CPU(
    id VARCHAR(36) DEFAULT uuid_generate_v4() NOT NULL, 
    name VARCHAR(255) NOT NULL,
    manufacturer VARCHAR(50) NOT NULL,
    cores INTEGER NOT NULL,
    cache VARCHAR(100) NOT NULL,
    speed REAL NOT NULL,
    tdp INTEGER NOT NULL,
    price REAL NOT NULL,
    PRIMARY KEY(id)
);