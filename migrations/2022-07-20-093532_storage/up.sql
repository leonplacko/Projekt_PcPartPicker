-- Your SQL goes here
CREATE TABLE Storages(
    id VARCHAR(36) DEFAULT uuid_generate_v4() NOT NULL,
    name VARCHAR(255) NOT NULL,
    manufacturer VARCHAR(50) NOT NULL,
    capacity INTEGER NOT NULL,
    speed INTEGER NOT NULL,
    type VARCHAR(10) NOT NULL,
    price REAL NOT NULL,
    PRIMARY KEY(id) 
);