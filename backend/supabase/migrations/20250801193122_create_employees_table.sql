create table if not exists users (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    email TEXT UNIQUE NOT NULL,
    age INTEGER,
    gender TEXT,
    joined_at TIMESTAMP,
    role TEXT
);

