-- Your SQL goes here
CREATE TABLE babies (
  id VARCHAR(50) PRIMARY KEY NOT NULL,
  name VARCHAR(100) NOT NULL,
  photo BLOB,
  sex VARCHAR(10) NOT NULL,
  birthday TEXT NOT NULL,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
)
