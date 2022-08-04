-- Your SQL goes here
CREATE TABLE Storage_slots(
    id VARCHAR(36) DEFAULT uuid_generate_v4() NOT NULL,
    motherboard_id VARCHAR(36) REFERENCES Motherboards(id) ON DELETE CASCADE,
    storage_id VARCHAR(36) REFERENCES Storages(id) ON DELETE CASCADE,
    slot VARCHAR(50) NOT NULL,
    PRIMARY KEY(id)

);