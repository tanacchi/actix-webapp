use actix_web::{
    web
};
use crate::state::{
    AppState, AppStateWithCounter
};
use crate::handlers;
use std::sync::Mutex;

pub fn app_config(config: &mut web::ServiceConfig) {
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });
    config.data(AppState {app_name: "Actix-web".to_string()})
        .app_data(counter.clone())
        .service(web::resource("/").route(web::get().to(handlers::index)))
        .service(web::resource("/echo.html").route(web::get().to(handlers::echo)))
        .service(web::resource("/form").route(web::get().to(handlers::form)))
        .service(web::resource("/register").route(web::post().to(handlers::register)))
        .route("/count", web::get().to(handlers::count))
        .route("/users", web::get().to(handlers::user_list))
        .route("/users/{user_name}", web::get().to(handlers::user_show))
        .route("categories", web::get().to(handlers::category_list))
        .route("/categories/new", web::get().to(handlers::category_form))
        .route("/categories/new", web::post().to(handlers::add_category));
}
