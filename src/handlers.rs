use actix_web::{
    web,
    HttpResponse,
    Responder, Result
};
use crate::state;
use crate::param;

pub async fn index(data: web::Data<state::AppState>) -> String {
    let app_name = &data.app_name;
    format!("Hello {}!", app_name)
}

pub async fn echo() -> impl Responder {
    "Wow"
}

pub async fn count(data: web::Data<state::AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;
    format!("Request number: {}", counter)
}

pub async fn form() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/form.html")))
}

use crate::{db, models::User, error::MyError};
use deadpool_postgres::{Client, Pool};
pub async fn register(params : web::Form<param::ParamsForRegister>, db_pool: web::Data<Pool>) -> Result<HttpResponse> {
    let user_info = User {name: params.name.clone()};
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let new_user = db::add_user(&client, user_info).await?;
    Ok(HttpResponse::Ok().json(new_user))
}
