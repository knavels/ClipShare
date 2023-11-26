CREATE TABLE IF NOT EXISTS clips
(
    id          TEXT PRIMARY KEY NOT NULL,
    short_code  TEXT UNIQUE NOT NULL,
    content     TEXT NOT NULL,
    title       TEXT,
    created_at  DATETIME NOT NULL,
    expires_at  DATETIME,
    password    TEXT,
    views       BIGINT NOT NULL
);