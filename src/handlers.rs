use actix_web::{
    web,
    HttpResponse,
    Responder, Result
};
use crate::state;
use crate::param;
use crate::templates;

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

pub async fn user_list(db_pool: web::Data<Pool>) -> Result<HttpResponse> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let users = db::get_all_users(&client).await?;
    Ok(HttpResponse::Ok().json(users))
}

pub async fn user_show(web::Path(user_name): web::Path<String>, db_pool: web::Data<Pool>) -> Result<HttpResponse> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let user = db::search_user(&client, user_name).await?;
    Ok(HttpResponse::Ok().json(user))
}

pub async fn category_list(db_pool: web::Data<Pool>) -> Result<HttpResponse> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let categories = db::get_all_categories(&client).await?;
    Ok(HttpResponse::Ok().json(categories))
}

pub async fn category_form() -> Result<HttpResponse> {
    let html: String = templates::category_form();
    Ok(HttpResponse::Ok().body(html))
}

use crate::{models::Category};
pub async fn add_category(params: web::Form<param::ParamsForNewCategory>, db_pool: web::Data<Pool>) -> Result<HttpResponse> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let _new_category = Category {name: params.name.clone()};
    let new_category = db::add_category(&client, _new_category).await?;
    Ok(HttpResponse::Ok().json(new_category))
}

pub async fn signup_form() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/form.html")))
}

use crate::{db, models::User, error::MyError};
use deadpool_postgres::{Client, Pool};
pub async fn signup(params : web::Form<param::ParamsForRegister>, db_pool: web::Data<Pool>) -> Result<HttpResponse> {
    let user_info = User {name: params.name.clone()};
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let new_user = db::add_user(&client, user_info).await?;
    Ok(HttpResponse::Ok().json(new_user))
}

pub async fn signin_form() -> Result<HttpResponse> {
    unimplemented!();
}

pub async fn signin() -> Result<HttpResponse> {
    unimplemented!();
}

pub async fn signout() -> Result<HttpResponse> {
    unimplemented!();
}
