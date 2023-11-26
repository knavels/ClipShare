CREATE TABLE IF NOT EXISTS clip_share
(
    id              TEXT PRIMARY KEY NOT NULL,
    short_code      TEXT UNIQUE NOT NULL,
    content         TEXT NOT NULL,
    title           TEXT,
    created_at      DATETIME NOT NULL,
    expires_at      DATETIME,
    password        TEXT,
    hits            BIGINT NOT NULL,
);