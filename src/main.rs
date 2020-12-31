use actix_web::{get, post, web, App,
                HttpResponse, HttpServer,
                Responder};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

async fn echo() -> impl Responder {
    "Wow"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
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
