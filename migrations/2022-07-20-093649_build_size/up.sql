-- Your SQL goes here
CREATE TABLE Build_size(
    id VARCHAR(36) DEFAULT uuid_generate_v4() NOT NULL,
    motherboard_id VARCHAR(36)  REFERENCES Motherboards(id) ON DELETE CASCADE,
    case_id VARCHAR(36)  REFERENCES Pc_case(id) ON DELETE CASCADE,
    size VARCHAR(50) NOT NULL,
    PRIMARY KEY(id)
);