-- Your SQL goes here
CREATE TABLE `configs`(
	`id` INTEGER NOT NULL PRIMARY KEY,
	`itchio_api_key` TEXT
);

CREATE TABLE `games`(
	`id` TEXT NOT NULL PRIMARY KEY,
	`title` TEXT NOT NULL
);

