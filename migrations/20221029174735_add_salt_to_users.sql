-- migrations/20221029174735_add_salt_to_users.sql
ALTER TABLE users ADD COLUMN salt TEXT NOT NULL;