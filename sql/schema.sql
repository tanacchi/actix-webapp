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

CREATE TABLE testing.reports (
  id          BIGSERIAL PRIMARY KEY,
  comment     VARCHAR(1500),
  date        VARCHAR(50) NOT NULL,
  category_id BIGSERIAL REFERENCES testing.categories(id),
  user_id     BIGSERIAL REFERENCES testing.users(id)
);
