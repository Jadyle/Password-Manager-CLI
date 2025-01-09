-- Your SQL goes here
CREATE TABLE users (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  user VARCHAR NOT NULL,
  label_account VARCHAR NOT NULL,
  salt_account TEXT NOT NULL,
  master_password TEXT NOT NULL
);

CREATE TABLE passwords (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  label VARCHAR NOT NULL,
  r_password TEXT NOT NULL,
  salt_password TEXT NOT NULL,
  nonce_password TEXT NOT NULL
);