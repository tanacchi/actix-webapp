use actix_web::{
    web, HttpResponse, Result
};
use crate::state;
use crate::param;
use crate::templates;

pub async fn index(data: web::Data<state::AppState>) -> String {
    let app_name = &data.app_name;
    format!("Hello {}!", app_name)
}

pub async fn dashboard(id: Identity) -> Result<HttpResponse> {
    let logged_in: bool = id.identity().is_some();
    let html: String = templates::dashboard(logged_in);
    Ok(HttpResponse::Ok().body(html))
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
    let _new_category = Category {id: -1, name: params.name.clone()};
    let new_category = db::add_category(&client, _new_category).await?;
    Ok(HttpResponse::Ok().json(new_category))
}

pub async fn signup_form() -> Result<HttpResponse> {
    let html: String = templates::signup_form();
    Ok(HttpResponse::Ok()
       .content_type("text/html; charset=utf-8")
       .body(html))
}

use crate::{
    db,
    models::User,
    error::{MyError, AccountError},
};
use deadpool_postgres::{Client, Pool};
pub async fn signup(params : web::Form<param::ParamsForSignUp>, db_pool: web::Data<Pool>) -> Result<HttpResponse> {
    let user_info = User {id: -1, name: params.name.clone()};
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let new_user = db::add_user(&client, user_info)
        .await
        .map_err(AccountError::SignUpFailed)?;
    Ok(HttpResponse::Ok().json(new_user))
}

pub async fn signin_form() -> Result<HttpResponse> {
    let html: String = templates::signin_form();
    Ok(HttpResponse::Ok().body(html))
}

use actix_identity::Identity;
pub async fn signin(params: web::Form<param::ParamsForSignIn>,
                    db_pool: web::Data<Pool>,
                    id: Identity) -> Result<HttpResponse> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let user = db::search_user(&client, params.name.clone())
        .await
        .map_err(|_err| AccountError::SignInFailed)?;  // FIXME
    id.remember(user.id.to_string());
    Ok(HttpResponse::Ok().json(user))
}

pub async fn signout(id: Identity) -> HttpResponse {
    id.forget();
    HttpResponse::Ok().finish()
}

pub async fn new_report_form(id: Identity, db_pool: web::Data<Pool>) -> Result<HttpResponse> {
    let logged_in: bool = id.identity().is_some();
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let categories: Vec<Category> = db::get_all_categories(&client).await?;
    let html: String = templates::report_form(logged_in, categories);
    Ok(HttpResponse::Ok().body(html))
}

pub async fn new_report(params: web::Form<param::ParamsForNewReport>,
                        db_pool: web::Data<Pool>,
                        id: Identity) -> Result<HttpResponse> {
    use chrono::{Date, NaiveDateTime, Local, TimeZone, LocalResult};
    use crate::models::Report;

    let date: Date<Local> = NaiveDateTime::parse_from_str(
        &format!("{} 00:00:00", &params.date),
        "%Y-%m-%d %H:%M:%S")
        .map(|dt| Local.from_local_datetime(&dt))
        .unwrap()
        .map(|dt| dt.date())
        .unwrap();

    // if date > Local::today() {
        // return MyError::InvalidDateOfReport;
    // }

    let user_id: i64 = id.identity()
        .and_then(|id_str| id_str.parse::<i64>().ok())
        .unwrap();
    let _new_report = Report {
        id: -1,
        user_id: 1,
        comment: params.comment.clone(),
        date: date.format("%Y-%m-%d").to_string(),
        category_id: params.category.clone()
        // user_id: user_id,
    };
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let new_report = db::add_report(&client, _new_report).await?;

    println!("comment: {}\ndate:{:?}\ncategory:{}",
             params.comment.clone(),
             date,
             // params.category.clone()
             1
    );

    Ok(HttpResponse::Ok().json(new_report))
}

pub async fn report_show(web::Path(report_id): web::Path<i64>,
                         db_pool: web::Data<Pool>) -> Result<HttpResponse> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let report = db::get_report(&client, report_id).await?;
    Ok(HttpResponse::Ok().json(report))
}
