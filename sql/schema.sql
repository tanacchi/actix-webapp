DROP SCHEMA IF EXISTS testing CASCADE;
CREATE SCHEMA testing;

CREATE TABLE testing.users (
  id      BIGSERIAL PRIMARY KEY,
  name    VARCHAR(200) NOT NULL,
  UNIQUE (name)
);
