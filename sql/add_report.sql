INSERT INTO testing.reports (comment, date, category_id, user_id)
VALUES ($1, $2, $3, $4)
RETURNING $table_fields
