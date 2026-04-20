-- Your SQL goes here
CREATE TABLE people (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    other_names TEXT OPTIONAL,
    phone_number TEXT,
    profile_picture_url TEXT DEFAULT '',
    email TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    verified BOOLEAN NOT NULL DEFAULT FALSE
)
