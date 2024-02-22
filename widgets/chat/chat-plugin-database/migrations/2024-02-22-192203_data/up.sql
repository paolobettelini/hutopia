CREATE TABLE chat_message (
    id SERIAL PRIMARY KEY,
    user_id UUID NOT NULL,
    message_text TEXT NOT NULL
);