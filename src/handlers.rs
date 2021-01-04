use serde::{Deserialize, Serialize};
use actix_web::{
    web,
    HttpResponse,
    Responder, Result
};
use crate::state;

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

#[derive(Serialize, Deserialize)]
pub struct ParamsForRegister {
    name: String,
}

pub async fn register(params : web::Form<ParamsForRegister>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/plain")
        .body(format!("Your name is {}", params.name)))
}
