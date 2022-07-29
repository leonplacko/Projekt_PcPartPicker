-- Your SQL goes here
CREATE TABLE Pc_case(
    id VARCHAR(36) DEFAULT uuid_generate_v4() NOT NULL,
    name VARCHAR(255) NOT NULL,
    manufacturer VARCHAR(50) NOT NULL,
    dimensions VARCHAR(50) NOT NULL,
    ventilators INTEGER NOT NULL,
    available_vent INTEGER NOT NULL,
    price REAL NOT NULL,
    PRIMARY KEY(id)
    
);