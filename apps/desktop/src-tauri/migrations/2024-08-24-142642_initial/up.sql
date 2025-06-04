CREATE TABLE `configs` (
  `id` INTEGER NOT NULL PRIMARY KEY CHECK (`id` = 0),
  `itchio_api_key` TEXT,
  `legacy_games_token` TEXT,
  `legacy_games_email` TEXT,
  `epic_games_refresh_token` TEXT
);

CREATE TABLE `games` (
  `id` TEXT NOT NULL,
  `source` TEXT CHECK (
    source IN ('itchio', 'legacy_games', 'epic_games')
  ) NOT NULL,
  `title` TEXT NOT NULL,
  `key` TEXT,
  `developer` TEXT,
  `launch_target` TEXT,
  `path` TEXT,
  `version` TEXT,
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
  `sort_title` TEXT NOT NULL,
  PRIMARY KEY (`id`, `source`)
);
