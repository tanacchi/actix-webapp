mod config;
mod db;
mod error;
mod handlers;
mod models;
mod param;
mod routes;
mod state;
mod templates;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{
        App,
        HttpServer,
        middleware::Logger,
    };
    use actix_identity::{
        CookieIdentityPolicy,
        IdentityService,
    };
    use actix_cors::Cors;
    use crate::routes;
    use dotenv::dotenv;
    use tokio_postgres::NoTls;

    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();
    dotenv().ok();

    let config = config::Config::from_env().unwrap();
    let pool = config.pg.create_pool(NoTls).unwrap();


    let server = HttpServer::new(move || {
        let cors = Cors::permissive();  // FIXME
        App::new()
            .wrap(Logger::default())
            .wrap(IdentityService::new(
                    CookieIdentityPolicy::new(&[0; 32])
                    .name("auth-cookie")
                    .secure(false)))
            .wrap(cors)
            .data(pool.clone())
            .configure(routes::app_config)

    })
    .bind(config.server_addr.clone())?
    .run();
    println!("Server running at http://{}/", config.server_addr);

    server.await
}
