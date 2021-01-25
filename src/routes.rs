use actix_web::{
    web
};
use crate::state::{
    AppState,
};
use crate::handlers;

pub fn app_config(config: &mut web::ServiceConfig) {
    config.data(AppState {app_name: "Actix-web".to_string()})
        .service(web::resource("/").route(web::get().to(handlers::index)))
        .route("/signup", web::get().to(handlers::signup_form))
        .route("/signup", web::post().to(handlers::signup))
        .route("/dashboard", web::get().to(handlers::dashboard))
        .route("/signin", web::get().to(handlers::signin_form))
        .route("/signin", web::post().to(handlers::signin))
        .route("/signout", web::get().to(handlers::signout))
        .route("/users", web::get().to(handlers::user_list))
        .route("/users/{user_name}", web::get().to(handlers::user_show))
        .route("/reports/new", web::get().to(handlers::new_report_form))
        .route("/reports/new", web::post().to(handlers::new_report))
        .route("/reports/{report_id}", web::get().to(handlers::report_show))
        .route("/categories", web::get().to(handlers::category_list))
        .route("/categories/new", web::get().to(handlers::category_form))
        .route("/categories/new", web::post().to(handlers::add_category));
}
