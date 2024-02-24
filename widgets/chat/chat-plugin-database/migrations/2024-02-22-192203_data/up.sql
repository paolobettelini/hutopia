CREATE TABLE chat_message (
    id SERIAL PRIMARY KEY,
    username TEXT NOT NULL,
    message_text TEXT NOT NULL
);