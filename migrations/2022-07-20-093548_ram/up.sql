-- Your SQL goes here
CREATE TABLE RAM(
    id VARCHAR DEFAULT uuid_generate_v4() NOT NULL,
    name VARCHAR(255) NOT NULL,
    manufacturer VARCHAR(50) NOT NULL,
    speed INTEGER NOT NULL,
    capacity INTEGER NOT NULL,
    price REAL NOT NULL,
    PRIMARY KEY(id)
);