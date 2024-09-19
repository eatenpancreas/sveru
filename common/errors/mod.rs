use serde::Serialize;
use thiserror::Error;
use ts_rs::TS;

mod into_response;
mod r#impl;
mod merged;
mod into;

pub use into_response::*;
pub use r#impl::*;
pub use merged::*;

#[derive(Error, Serialize, TS, Debug)]
#[error("{message}")]
pub struct ServerError {
  pub message: String,
  pub code: &'static str,
  pub source: Option<String>,
}

#[derive(Error, Serialize, TS, Debug)]
#[error("{message}")]
pub struct AuthenticationError {
  pub message: String
}

#[derive(Error, Serialize, TS, Debug)]
#[error("{message}")]
pub struct AuthorizationError {
  pub message: String
}

#[derive(Error, Serialize, TS, Debug)]
#[error("{message}")]
pub struct FieldError {
  pub field: String,
  pub message: String
}

#[derive(Error, Serialize, TS, Debug)]
#[error("{message}")]
pub struct UserError {
  pub message: String
}


