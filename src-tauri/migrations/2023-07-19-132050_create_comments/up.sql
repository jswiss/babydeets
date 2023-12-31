-- Your SQL goes here
CREATE TABLE IF NOT EXISTS comments (
  id VARCHAR(50) PRIMARY KEY NOT NULL,
  note_id VARCHAR(50) NOT NULL,
  comment TEXT,
  image BLOB,
  file BLOB,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (note_id)
    REFERENCES notes (id)
)
