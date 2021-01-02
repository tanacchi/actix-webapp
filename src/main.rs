use serde::{Deserialize, Serialize};
use actix_web::{get, post, web, App,
                HttpResponse, HttpServer,
                Responder, Result};
use std::sync::Mutex;


struct AppState {
    app_name: String,
}

struct AppStateWithCounter {
    counter: Mutex<i32>,
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name;
    format!("Hello {}!", app_name)
}

async fn echo() -> impl Responder {
    "Wow"
}

async fn count(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;
    format!("Request number: {}", counter)
}

async fn form() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
           .content_type("text/html; charset=utf-8")
           .body(include_str!("../static/form.html")))
}

#[derive(Serialize, Deserialize)]
pub struct ParamsForRegister {
    name: String,
}

async fn register(params : web::Form<ParamsForRegister>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/plain")
        .body(format!("Your name is {}", params.name)))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });
    HttpServer::new(move || {
        App::new()
            .data(AppState {
                app_name: String::from("Actix-web"),
            })
            .service(index)
            .service(
                web::scope("/ahiahi")
                    .route("/echo.html", web::get().to(echo)),
            )
            .service(
                web::resource("/form").route(web::get().to(form))
            )
            .service(
                web::resource("/register").route(web::post().to(register))
            )
            .app_data(counter.clone())
            .route("/count", web::get().to(count))
    })
    .bind("localhost:8080")?
    .run()
    .await
}
