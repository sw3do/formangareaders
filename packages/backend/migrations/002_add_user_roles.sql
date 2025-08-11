CREATE TYPE user_role AS ENUM ('user', 'moderator', 'admin');

ALTER TABLE users ADD COLUMN role user_role DEFAULT 'user' NOT NULL;

CREATE INDEX idx_users_role ON users(role);