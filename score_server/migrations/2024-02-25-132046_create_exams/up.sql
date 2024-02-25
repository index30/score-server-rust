CREATE TABLE exams(
    id BIGSERIAL PRIMARY KEY,
    --------
    name VARCHAR NOT NULL,
    description VARCHAR NOT NULL UNIQUE
);