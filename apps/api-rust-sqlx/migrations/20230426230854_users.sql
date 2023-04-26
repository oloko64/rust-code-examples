-- Add migration script here
CREATE TABLE IF NOT EXISTS "users" (
	"id"	INTEGER NOT NULL UNIQUE,
	"name"	TEXT,
	PRIMARY KEY("id" AUTOINCREMENT)
);