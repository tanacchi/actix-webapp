use actix_web::{get, post, web, App,
                HttpResponse, HttpServer,
                Responder};
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
            .app_data(counter.clone())
            .route("/count", web::get().to(count))
    })
    .bind("localhost:8080")?
    .run()
    .await
}
