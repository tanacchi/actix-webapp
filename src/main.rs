mod config;
mod handlers;
mod state;
mod param;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{
        App,
        HttpServer,
    };
    use crate::routes;

    HttpServer::new(move || {
        App::new()
            .configure(routes::app_config)
    })
    .bind("localhost:8080")?
    .run()
    .await
}
