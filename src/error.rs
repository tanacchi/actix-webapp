use actix_web::{HttpResponse, ResponseError};
use deadpool_postgres::PoolError;
use derive_more::{Display, From};
use tokio_pg_mapper::Error as PGMError;
use tokio_postgres::error::Error as PGError;
use log::{warn};

#[derive(Display, From, Debug)]
pub enum MyError {
    NotFound,
    InvalidDateOfReport,
    PGError(PGError),
    PGMError(PGMError),
    PoolError(PoolError),
}

impl std::error::Error for MyError {}

impl ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            MyError::NotFound => {
                warn!("NotFound");
                HttpResponse::NotFound().finish()
            },
            MyError::InvalidDateOfReport => {
                warn!("InvalidDateOfReport");
                HttpResponse::BadRequest().finish()
            },
            MyError::PGError(ref err) => {
                warn!("PGError: {}", err);
                HttpResponse::InternalServerError().body(err.to_string())
            },
            MyError::PoolError(ref err) => {
                warn!("PoolError: {}", err);
                HttpResponse::InternalServerError().body(err.to_string())
            },
            MyError::PGMError(ref err) => {
                warn!("PGMError: {}", err);
                HttpResponse::InternalServerError().finish()
            },
        }
    }
}

#[derive(Display, From, Debug)]
pub enum AccountError {
    SignUpFailed(MyError),
    SignInFailed,
}

impl std::error::Error for AccountError {}

impl ResponseError for AccountError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            AccountError::SignUpFailed(_) => {
                warn!("SignUpFailed");
                HttpResponse::BadRequest().finish()
            },
            AccountError::SignInFailed => {
                warn!("SignInFailed");
                HttpResponse::Unauthorized().finish()
            }
        }
    }
}
