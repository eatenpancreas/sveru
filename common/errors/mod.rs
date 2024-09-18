use serde::Serialize;
use thiserror::Error;
use ts_rs::TS;

mod into_response;

#[derive(Error, Serialize)]
pub(crate) enum ApiError {
  #[error(transparent)]
  ServerError(#[from] ServerError),
  #[error(transparent)]
  AuthenticationError(#[from] AuthenticationError),
  #[error(transparent)]
  AuthorizationError(#[from] AuthorizationError),
  #[error(transparent)]
  FieldError(#[from] FieldError),
  #[error(transparent)]
  FatalError(#[from] FatalError)
}

#[derive(Error, Serialize, TS)]
#[error("{message}")]
pub struct ServerError {
  message: String,
  code: u16
}

#[derive(Error, Serialize, TS)]
#[error("{message}")]
pub struct AuthenticationError {
  message: String
}

#[derive(Error, Serialize, TS)]
#[error("{message}")]
pub struct AuthorizationError {
  message: String
}

#[derive(Error, Serialize, TS)]
#[error("{message}")]
pub struct FieldError {
  field: String,
  message: String
}

#[derive(Error, Serialize, TS)]
#[error("{message}")]
pub struct FatalError {
  message: String
}


