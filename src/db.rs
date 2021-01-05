use crate::models::User;
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

pub async fn add_user(client: &Client, user_info: User) -> User {
    let _stmt = include_str!("../sql/add_user.sql");
    let _stmt = _stmt.replace("$table_fields", &User::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.unwrap();

    client.query(&stmt, &[&user_info.name])
        .await
        .iter()
        .map(|row| User::from_row_ref(&row[0]).unwrap())
        .collect::<Vec<User>>()
        .pop()
        .unwrap()
}
