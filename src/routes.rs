use actix_web::{
    web
};
use crate::state::{
    AppState, AppStateWithCounter
};
use crate::handlers::{
    index, echo, form, register, count
};
use std::sync::Mutex;

pub fn app_config(config: &mut web::ServiceConfig) {
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });
    config.data(AppState {app_name: "Actix-web".to_string()})
        .app_data(counter.clone())
        .service(web::resource("/").route(web::get().to(index)))
        .service(web::resource("/echo.html").route(web::get().to(echo)))
        .service(web::resource("/form").route(web::get().to(form)))
        .service(web::resource("/register").route(web::post().to(register)))
        .route("/count", web::get().to(count));
}
