CREATE TABLE IF NOT EXISTS pacman
(
    userid      TEXT    NOT NULL PRIMARY KEY,
    packages    INTEGER NOT NULL DEFAULT 0,
    last_update TEXT    NOT NULL DEFAULT (datetime('now'))
)