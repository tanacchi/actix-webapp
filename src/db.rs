use crate::{models::User, models::Category, error::MyError};
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

pub async fn add_user(client: &Client, user_info: User) -> Result<User, MyError> {
    let _stmt = include_str!("../sql/add_user.sql");
    let _stmt = _stmt.replace("$table_fields", &User::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.unwrap();

    client.query(&stmt, &[&user_info.name])
        .await?
        .iter()
        .map(|row| User::from_row_ref(row).unwrap())
        .collect::<Vec<User>>()
        .pop()
        .ok_or(MyError::NotFound)
}

pub async fn get_all_users(client: &Client) -> Result<Vec<User>, MyError> {
    let _stmt = include_str!("../sql/get_all_users.sql");
    let _stmt = _stmt.replace("$table_fields", &User::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.unwrap();
    Ok(client
       .query(&stmt, &[])
       .await?
       .iter()
       .map(|row| User::from_row_ref(row).unwrap())
       .collect::<Vec<User>>())
}

pub async fn search_user(client: &Client, name: String) -> Result<User, MyError> {
    let _stmt = include_str!("../sql/search_user.sql");
    let _stmt = _stmt.replace("$table_fields", &User::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.unwrap();
    let _user = client.query_one(&stmt, &[&name]).await?;
    Ok(User::from_row_ref(&_user)?)
}

pub async fn get_all_categories(client: &Client) -> Result<Vec<Category>, MyError> {
    let _stmt = include_str!("../sql/get_all_categories.sql");
    let _stmt = _stmt.replace("$table_fields", &User::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.unwrap();
    Ok(client
       .query(&stmt, &[])
       .await?
       .iter()
       .map(|row| Category::from_row_ref(row).unwrap())
       .collect::<Vec<Category>>())
}
