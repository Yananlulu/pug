CREATE TABLE cards(
  id BIGINT NOT NULL AUTO_INCREMENT,
  title VARCHAR(255) NOT NULL,
  body TEXT NOT NULL,
  media_type VARCHAR(8) NOT NULL,
  action VARCHAR(32) NOT NULL,
  href VARCHAR(255) NOT NULL,
  logo VARCHAR(255) NOT NULL,
  loc VARCHAR(16) NOT NULL,
  lang VARCHAR(8) NOT NULL,
  position TINYINT NOT NULL,
  created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at DATETIME NOT NULL
);
CREATE INDEX idx_cards_lang ON cards(lang);
CREATE INDEX idx_cards_loc ON cards(loc);

CREATE TABLE links(
  id BIGINT NOT NULL AUTO_INCREMENT,
  href VARCHAR(255) NOT NULL,
  label VARCHAR(255) NOT NULL,
  loc VARCHAR(16) NOT NULL,
  lang VARCHAR(8) NOT NULL,
  x TINYINT NOT NULL,
  y TINYINT NOT NULL,
  created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at DATETIME NOT NULL
);
CREATE INDEX idx_links_lang ON links(lang);
CREATE INDEX idx_links_loc ON links(loc);

CREATE TABLE friend_links(
  id BIGINT NOT NULL AUTO_INCREMENT,
  title VARCHAR(255) NOT NULL,
  home VARCHAR(255) NOT NULL,
  logo VARCHAR(255) NOT NULL,
  lang VARCHAR(8) NOT NULL,
  position TINYINT NOT NULL,
  created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at DATETIME NOT NULL
);
CREATE INDEX idx_friend_links_lang ON friend_links(lang);

CREATE TABLE leave_words(
  id BIGINT NOT NULL AUTO_INCREMENT,
  body TEXT NOT NULL,
  media_type VARCHAR(8) NOT NULL,
  created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at DATETIME NOT NULL
);

CREATE TABLE votes(
  id BIGINT NOT NULL AUTO_INCREMENT,
  `point` BITINT NOT NULL,
  resource_type VARCHAR(255) NOT NULL,
  resource_id BIGINT NOT NULL,
  created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at DATETIME NOT NULL
);
CREATE INDEX idx_votes_resource_type ON votes(resource_type);
CREATE UNIQUE INDEX idx_votes_resource ON votes(resource_type, resource_id);
