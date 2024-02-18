CREATE TABLE users (
  --id         SERIAL PRIMARY KEY,
  id         TEXT PRIMARY KEY,
  email      TEXT NOT NULL UNIQUE,
  username   TEXT NOT NULL UNIQUE,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Session tokens
-- TODO: these are here forever
CREATE TABLE user_tokens (
  user_id  TEXT NOT NULL,
  token    TEXT NOT NULL,
  PRIMARY KEY (user_id, token),
  FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE  
);

-- TODO: popoulate
CREATE TABLE google_tokens (
  id SERIAL PRIMARY KEY,
  user_id TEXT NOT NULL UNIQUE,
  access_secret TEXT NOT NULL,
  -- refresh_secret TEXT NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);