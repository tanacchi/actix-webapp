INSERT INTO testing.users(name)
VALUES ($1)
RETURNING $table_fields;
