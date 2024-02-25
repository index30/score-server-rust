CREATE TABLE users(
    id BIGSERIAL PRIMARY KEY,
    --------
    name VARCHAR NOT NULL,
    service_name VARCHAR NOT NULL UNIQUE
);
