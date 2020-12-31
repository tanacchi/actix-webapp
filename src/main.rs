use actix_web::{get, post, web, App,
                HttpResponse, HttpServer,
                Responder};

struct AppState {
    app_name: String,
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name;
    format!("Hello {}!", app_name)
}

async fn echo() -> impl Responder {
    "Wow"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .data(AppState {
                app_name: String::from("Actix-web"),
            })
            .service(index)
            .service(
                web::scope("/ahiahi")
                    .route("/echo.html", web::get().to(echo)),
            )
    })
    .bind("localhost:8080")?
    .run()
    .await
}
