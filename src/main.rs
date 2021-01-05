mod config;
mod handlers;
mod state;
mod param;
mod routes;
mod models;
mod db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{
        App,
        HttpServer,
    };
    use crate::routes;
    use dotenv::dotenv;
    use tokio_postgres::NoTls;

    dotenv().ok();

    let config = config::Config::from_env().unwrap();
    let pool = config.pg.create_pool(NoTls).unwrap();

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .configure(routes::app_config)
    })
    .bind("localhost:8080")?
    .run()
    .await
}
