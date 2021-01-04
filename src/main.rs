mod handlers;
mod state;
mod config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{
        App,
        HttpServer,
    };
    use crate::config;

    HttpServer::new(move || {
        App::new()
            .configure(config::app_config)
    })
    .bind("localhost:8080")?
    .run()
    .await
}
