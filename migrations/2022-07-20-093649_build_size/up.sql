-- Your SQL goes here
CREATE TABLE Build_size(
    motherboard_id VARCHAR(36) NOT NULL REFERENCES Motherboards(id),
    case_id VARCHAR(36) NOT NULL REFERENCES Pc_case(id),
    size VARCHAR(50) NOT NULL,
    PRIMARY KEY(motherboard_id, case_id)
);