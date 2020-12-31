use actix_web::{get, post, web, App,
                HttpResponse, HttpServer,
                Responder};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
    })
    .bind("localhost:8080")?
    .run()
    .await
}
