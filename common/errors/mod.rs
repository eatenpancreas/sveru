use serde::{Serialize};
use thiserror::Error;
use ts_rs::TS;

mod into_response;
mod r#impl;
mod merged;
mod err_into_sveru_err;
mod drain;
mod from_merged;
mod collect_errs;

pub use err_into_sveru_err::*;
pub use drain::*;
pub use merged::*;
pub use collect_errs::*;

#[derive(Error, Serialize, TS, Debug, Clone)]
#[error("{message}")]
pub struct ServerError {
  pub message: String,
  pub code: &'static str,
}

#[derive(Error, Serialize, TS, Debug, Clone)]
#[error("{message}")]
pub struct AuthenticationError {
  pub message: String
}

#[derive(Error, Serialize, TS, Debug, Clone)]
#[error("{message}")]
pub struct AuthorizationError {
  pub message: String
}

#[derive(Error, Serialize, TS, Debug, Clone)]
#[error("{message}")]
pub struct FieldError {
  pub field: String,
  pub message: String
}

#[derive(Error, Serialize, TS, Debug, Clone)]
#[error("{message}")]
pub struct UserError {
  pub message: String
}
