CREATE TABLE relay_users (
  --id         SERIAL PRIMARY KEY,
  id         TEXT PRIMARY KEY, -- token given by google
  email      TEXT NOT NULL UNIQUE,
  username   TEXT NOT NULL UNIQUE,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Session tokens
-- TODO: these are here forever
CREATE TABLE relay_user_tokens (
  user_id  TEXT NOT NULL,
  token    TEXT NOT NULL,
  PRIMARY KEY (user_id, token),
  FOREIGN KEY (user_id) REFERENCES relay_users(id) ON DELETE CASCADE  
);

-- TODO: popoulate
CREATE TABLE relay_google_tokens (
  id SERIAL PRIMARY KEY,
  user_id TEXT NOT NULL UNIQUE,
  access_secret TEXT NOT NULL,
  -- refresh_secret TEXT NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (user_id) REFERENCES relay_users(id) ON DELETE CASCADE
);