use axum::http::StatusCode;
use axum_thiserror::ErrorStatus;
use deadpool_diesel::{InteractError, PoolError};
use thiserror::Error;

#[derive(Debug, Error, ErrorStatus)]
pub enum AppError {
    #[error("API returned an error")]
    #[status(StatusCode::INTERNAL_SERVER_ERROR)]
    PgError(#[from] PoolError),
    #[error("API returned an error")]
    #[status(StatusCode::INTERNAL_SERVER_ERROR)]
    DeadPoolError(#[from] InteractError),
    #[error("API returned an error")]
    #[status(StatusCode::INTERNAL_SERVER_ERROR)]
    DieselError(#[from] diesel::result::Error),
    #[error("Incorrect request")]
    #[status(axum::http::StatusCode::BAD_REQUEST)]
    BadRequest,
    #[error("Invalid code, discount is not valid for this data")]
    #[status(axum::http::StatusCode::BAD_REQUEST)]
    CodeInvalid,
}
