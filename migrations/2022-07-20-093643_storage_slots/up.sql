-- Your SQL goes here
CREATE TABLE Storage_slots(
    motherboard_id VARCHAR(36) NOT NULL REFERENCES Motherboards(id),
    storage_id VARCHAR(36) NOT NULL REFERENCES Storages(id),
    slot VARCHAR(50) NOT NULL,
    PRIMARY KEY(motherboard_id, storage_id)

);