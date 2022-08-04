-- Your SQL goes here
CREATE TABLE RAM_slot(
    id VARCHAR(36) DEFAULT uuid_generate_v4() NOT NULL,
    motherboard_id VARCHAR(36) REFERENCES Motherboards(id) ON DELETE CASCADE,
    ram_id VARCHAR(36) REFERENCES RAM(id) ON DELETE CASCADE,
    type VARCHAR(50) NOT NULL,
    PRIMARY KEY(id)
);