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