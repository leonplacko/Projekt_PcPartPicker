-- Your SQL goes here
CREATE TABLE Socket(
    motherboard_id VARCHAR(36) NOT NULL REFERENCES Motherboards(id),
    cpu_id VARCHAR(36) NOT NULL REFERENCES CPU(id),
    socket_type VARCHAR(50) NOT NULL,
    PRIMARY KEY(motherboard_id, cpu_id)
);