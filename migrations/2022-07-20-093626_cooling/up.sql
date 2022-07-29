-- Your SQL goes here
CREATE TABLE Cooling(
    id VARCHAR(36) DEFAULT uuid_generate_v4() NOT NULL,
    name VARCHAR(255) NOT NULL,
    manufacturer VARCHAR(50) NOT NULL,
    ventilators INTEGER,
    cpu_ventilator BOOLEAN,
    price REAL NOT NULL,
    PRIMARY KEY(id)

);