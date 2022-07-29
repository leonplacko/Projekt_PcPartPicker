-- Your SQL goes here
CREATE TABLE Motherboards(
    id VARCHAR(36) DEFAULT uuid_generate_v4() NOT NULL,
    name VARCHAR(255) NOT NULL,
    manufacturer VARCHAR(50) NOT NULL,
    ram_slots INTEGER NOT NULL,
    sata_slots INTEGER NOT NULL,
    nvme_slots INTEGER NOT NULL,
    price REAL NOT NULL,
    PRIMARY KEY(id)
);