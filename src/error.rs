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
    ClientError(ClientError),
}

#[derive(Clone, Debug, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum ClientError {
    BadRequest,
    Unauthorized,
    Forbidden,
    NotFound,
    Conflict,
}

impl core::fmt::Display for ServerError {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        println!("->> {:<12} - {self:?}", "INTO_RES");

        let mut response = match self.clone() {
            ServerError::CreateAccountFail => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Create Account Fail").into_response()
            }
            ServerError::ClientError(client_error) => client_error.into_response(),
        };
        response.extensions_mut().insert(self);

        response
    }
}

impl IntoResponse for ClientError {
    fn into_response(self) -> Response {
        println!("->> {:<12} - {self:?}", "INTO_RES");

        let mut response = match &self {
            ClientError::BadRequest => (StatusCode::BAD_REQUEST, "Bad Request").into_response(),
            ClientError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
            ClientError::Forbidden => todo!(),
            ClientError::NotFound => todo!(),
            ClientError::Conflict => todo!(),
        };

        response.extensions_mut().insert(self);

        response
    }
}
