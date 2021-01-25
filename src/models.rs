use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "users")]
pub struct User {
    pub id: i64,
    pub name: String,
}

#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "categories")]
pub struct Category {
    pub id: i64,
    pub name: String,
}

#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "reports")]
pub struct Report {
    pub id: i64,
    pub comment: String,
    pub date: String,
    pub category_id: i64,
    pub user_id: i64
}
