CREATE TABLE locales(
  id BIGINT NOT NULL AUTO_INCREMENT PRIMARY KEY,
  lang VARCHAR(8) NOT NULL,
  code VARCHAR(255) NOT NULL,
  message TEXT NOT NULL,
  created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at DATETIME NOT NULL
);

CREATE INDEX idx_locales_lang ON locales(lang);
CREATE INDEX idx_locales_code ON locales(code);
CREATE UNIQUE INDEX idx_locales_lang_code ON locales(lang, code);
