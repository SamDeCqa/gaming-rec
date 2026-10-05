-- Add up migration script here
CREATE TABLE IF NOT EXISTS games (
    id UUID PRIMARY KEY NOT NULL UNIQUE DEFAULT gen_random_uuid(),
    name TEXT NOT NULL UNIQUE,
    creator TEXT NOT NULL,
    plays INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NULL DEFAULT NOW()
);