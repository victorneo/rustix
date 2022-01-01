-- Add migration script here
ALTER TABLE users
ADD COLUMN superuser boolean default false;