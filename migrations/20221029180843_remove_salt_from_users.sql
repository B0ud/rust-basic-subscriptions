-- migration 20221029180843_remove_salt_from_users.sql script here
ALTER TABLE users DROP COLUMN salt;