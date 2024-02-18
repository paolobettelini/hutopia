CREATE TABLE users (
  --id         SERIAL PRIMARY KEY,
  id         TEXT PRIMARY KEY,
  email      TEXT NOT NULL UNIQUE,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

--CREATE TABLE user_permissions (
--  id       SERIAL PRIMARY KEY,
--  user_id  INTEGER NOT NULL,
--  token    TEXT NOT NULL,
--  FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
--);

CREATE TABLE google_tokens (
  id SERIAL PRIMARY KEY,
  user_id TEXT NOT NULL UNIQUE,
  access_secret TEXT NOT NULL,
  -- refresh_secret TEXT NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);