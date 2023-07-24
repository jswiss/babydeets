-- Your SQL goes here
CREATE TABLE IF NOT EXISTS "notes" (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  FOREIGN KEY("baby_id") REFERENCES "babies",
  date TEXT NOT NULL,
  note TEXT,
  image BLOB,
  file BLOB
);
