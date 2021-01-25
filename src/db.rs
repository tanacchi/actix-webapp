use crate::{
    models::{User, Category, Report},
    error::MyError
};
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

pub async fn add_category(client: &Client, caterogy: Category) -> Result<Category, MyError> {
    let _stmt = include_str!("../sql/add_category.sql");
    let _stmt = _stmt.replace("$table_fields", &Category::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.unwrap();

    client.query(&stmt, &[&caterogy.name])
        .await?
        .iter()
        .map(|row| Category::from_row_ref(row).unwrap())
        .collect::<Vec<Category>>()
        .pop()
        .ok_or(MyError::NotFound)
}

pub async fn add_report(client: &Client, report: Report) -> Result<Report, MyError> {
    let _stmt = include_str!("../sql/add_report.sql");
    let _stmt = _stmt.replace("$table_fields", &Report::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.unwrap();

    let result = client.query(&stmt,
                 &[
                     &report.comment,
                     &report.date,
                     &report.category_id,
                     &report.user_id
                 ])
        .await;

    match result {
        Ok(report_vec) => {
            report_vec.iter()
                .map(|row| Report::from_row_ref(row).unwrap())
                .collect::<Vec<Report>>()
                .pop()
                .ok_or(MyError::NotFound)
        },
        Err(err) => {
            println!("ERROR: {}", err);
            Err(MyError::NotFound)
        }
    }
}

pub async fn get_report(client: &Client, report_id: i64) -> Result<Report, MyError> {
    let _stmt = include_str!("../sql/get_report.sql");
    let _stmt = _stmt.replace("$table_fields", &Report::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.unwrap();
    let report = client.query_one(&stmt, &[&report_id]).await?;
    Ok(Report::from_row_ref(&report)?)
}
