-- Your SQL goes here
CREATE TABLE `configs` (
  `id` INTEGER NOT NULL PRIMARY KEY,
  `itchio_api_key` TEXT
);

CREATE TABLE `games` (
  `id` TEXT NOT NULL,
  `source` TEXT NOT NULL,
  `title` TEXT NOT NULL,
  `key` TEXT,
  `developer` TEXT,
  `launch_target` TEXT,
  `path` TEXT,
  PRIMARY KEY (`id`, `source`)
);
