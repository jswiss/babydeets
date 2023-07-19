-- Your SQL goes here
CREATE TABLE IF NOT EXISTS "comments" (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  FOREIGN KEY("note_id") REFERENCES "notes",
  timestamp TEXT NOT NULL,
  note TEXT,
  image BLOB,
  file BLOB
)
