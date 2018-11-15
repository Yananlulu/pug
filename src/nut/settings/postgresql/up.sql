CREATE TABLE settings(
  id BIGSERIAL PRIMARY KEY,
  key VARCHAR(255) NOT NULL,
  value BYTEA NOT NULL,
  salt BYTEA,
  updated_at TIMESTAMP NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX idx_settings_key ON settings(key);
