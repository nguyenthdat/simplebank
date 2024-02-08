use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;
pub type SQLResult<T> = std::result::Result<T, sqlx::Error>;
pub type ServerResult<T> = std::result::Result<T, ServerError>;

#[derive(Clone, Debug, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum ServerError {
    CreateAccountFail,
}

impl core::fmt::Display for ServerError {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        println!("->> {:<12} - {self:?}", "INTO_RES");

        // Create a placeholder Axum reponse.
        let mut response = (StatusCode::BAD_REQUEST, "test 1234").into_response();

        // Insert the Error into the reponse.
        response.extensions_mut().insert(self);

        response
    }
}
