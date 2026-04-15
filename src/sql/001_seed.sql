CREATE TABLE IF NOT EXISTS messages (
    message_id     INTEGER PRIMARY KEY AUTOINCREMENT,
    conversation_id INTEGER NOT NULL,
    sender_id       INTEGER NOT NULL,
    content         TEXT NOT NULL,
    time_stamp      TEXT NOT NULL,
    status          TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS subscribers (
    subscriber_id INTEGER PRIMARY KEY
);

CREATE TABLE IF NOT EXISTS chat_room (
    chat_room_id INTEGER PRIMARY KEY
);

CREATE TABLE IF NOT EXISTS chat_room_subscribers (
    chat_room_id       INTEGER NOT NULL,
    subscriber_id  INTEGER NOT NULL,
    PRIMARY KEY (chat_room_id, subscriber_id),
    FOREIGN KEY (chat_room_id) REFERENCES conversations (chat_room_id),
    FOREIGN KEY (subscriber_id) REFERENCES subscribers (subscriber_id)
);
