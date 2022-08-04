-- Your SQL goes here
CREATE TABLE Socket(
    id VARCHAR(36) DEFAULT uuid_generate_v4() NOT NULL,
    motherboard_id VARCHAR(36) REFERENCES Motherboards(id) ON DELETE CASCADE,
    cpu_id VARCHAR(36) REFERENCES CPU(id) ON DELETE CASCADE,
    socket_type VARCHAR(50) NOT NULL,
    PRIMARY KEY(id)
);