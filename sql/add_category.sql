INSERT INTO testing.categories(name)
VALUES ($1)
RETURNING $table_fields
