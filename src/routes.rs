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
        .service(web::resource("/signup").route(web::get().to(handlers::signup_form)))
        .service(web::resource("/signup").route(web::post().to(handlers::signup)))
        .route("/dashboard", web::get().to(handlers::dashboard))
        .route("/signin", web::get().to(handlers::signin_form))
        .route("/signin", web::post().to(handlers::signin))
        .route("/signout", web::get().to(handlers::signout))
        .route("/count", web::get().to(handlers::count))
        .route("/users", web::get().to(handlers::user_list))
        .route("/users/{user_name}", web::get().to(handlers::user_show))
        .route("categories", web::get().to(handlers::category_list))
        .route("/categories/new", web::get().to(handlers::category_form))
        .route("/categories/new", web::post().to(handlers::add_category));
}
