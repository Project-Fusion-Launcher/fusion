PRAGMA foreign_keys = ON;

CREATE TABLE
    `configs` (
        `id` INTEGER NOT NULL PRIMARY KEY CHECK (`id` = 0),
        `it_api_key` TEXT,
        `lg_token` TEXT,
        `lg_email` TEXT,
        `eg_refresh_token` TEXT
    );

INSERT INTO
    `configs` (`id`)
VALUES
    (0);

CREATE TRIGGER `prevent_config_deletion` BEFORE DELETE ON `configs` FOR EACH ROW WHEN OLD.id = 0 BEGIN
SELECT
    RAISE (ABORT, 'Deletion of this row is not allowed');

END;

CREATE TABLE
    `games` (
        `id` TEXT,
        `source` TEXT CHECK (source IN ('it', 'lg', 'eg')) NOT NULL,
        `name` TEXT NOT NULL,
        `sort_name` TEXT NOT NULL,
        `developer` TEXT,
        `status` TEXT CHECK (
            status IN (
                'installed',
                'not_installed',
                'downloading',
                'installing',
                'uninstalling'
            )
        ) NOT NULL,
        `favorite` BOOLEAN NOT NULL DEFAULT FALSE,
        `hidden` BOOLEAN NOT NULL DEFAULT FALSE,
        `cover_url` TEXT,
        PRIMARY KEY (`id`, `source`)
    );