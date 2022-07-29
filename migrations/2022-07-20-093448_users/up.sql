-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE Users(
    id VARCHAR(36) DEFAULT uuid_generate_v4() PRIMARY KEY NOT NULL,
    username VARCHAR NOT NULL,
    password VARCHAR NOT NULL,
    isadmin BOOLEAN NOT NULL
);