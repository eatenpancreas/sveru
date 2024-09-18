use serde::Serialize;
use thiserror::Error;
use ts_rs::TS;

mod into_response;
mod r#impl;

pub use into_response::*;
pub use r#impl::*;

#[derive(Error, Serialize)]
#[serde(untagged)]
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
  UserError(#[from] UserError)
}

#[derive(Error, Serialize, TS)]
#[error("{message}")]
pub struct ServerError {
  pub message: String,
  pub code: u16
}

#[derive(Error, Serialize, TS)]
#[error("{message}")]
pub struct AuthenticationError {
  pub message: String
}

#[derive(Error, Serialize, TS)]
#[error("{message}")]
pub struct AuthorizationError {
  pub message: String
}

#[derive(Error, Serialize, TS)]
#[error("{message}")]
pub struct FieldError {
  pub field: String,
  pub message: String
}

#[derive(Error, Serialize, TS)]
#[error("{message}")]
pub struct UserError {
  pub message: String
}


