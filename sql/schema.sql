DROP SCHEMA IF EXISTS testing CASCADE;
CREATE SCHEMA testing;

CREATE TABLE testing.users (
  id      BIGSERIAL PRIMARY KEY,
  name    VARCHAR(200) NOT NULL,
  UNIQUE (name)
);

CREATE TABLE testing.categories (
  id      BIGSERIAL PRIMARY KEY,
  name    VARCHAR(50) NOT NULL UNIQUE
);
