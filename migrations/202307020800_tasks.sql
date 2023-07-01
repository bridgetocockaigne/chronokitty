CREATE TABLE IF NOT EXISTS tasks (
    id          INTEGER PRIMARY KEY NOT NULL,
    name        TEXT                NOT NULL,
    description TEXT                NOT NULL,
    labels      TEXT                NOT NULL,
    start_date  TEXT                NOT NULL,
    end_date    TEXT
);
