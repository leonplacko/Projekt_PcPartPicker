-- Your SQL goes here
CREATE TABLE RAM_slot(
    motherboard_id VARCHAR(36) NOT NULL REFERENCES Motherboards(id),
    ram_id VARCHAR(36) NOT NULL REFERENCES RAM(id),
    type VARCHAR(50) NOT NULL,
    PRIMARY KEY(motherboard_id, ram_id)
);