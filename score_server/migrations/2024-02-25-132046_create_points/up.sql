CREATE TABLE points(
    id BIGSERIAL PRIMARY KEY,
    --------
    users_id BIGINT NOT NULL,
    FOREIGN KEY (users_id) REFERENCES users(id),
    exams_id BIGINT NOT NULL,
    FOREIGN KEY (exams_id) REFERENCES exams(id),
    --------
    entrance_at TIMESTAMP WITH TIME ZONE NOT NULL
);