CREATE TABLE logs(
  id INTEGER PRIMARY KEY NOT NULL,
  message TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
