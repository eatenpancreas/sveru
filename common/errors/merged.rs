use serde::Serialize;
use thiserror::Error;
use ts_rs::TS;
use super::*;

#[derive(Error, Serialize, Debug, TS)]
#[serde(untagged)]
pub enum ApiError {
  #[error(transparent)]
  ServerError(#[from] ServerError),
  #[error(transparent)]
  AuthenticationError(#[from] AuthenticationError),
  #[error(transparent)]
  AuthorizationError(#[from] AuthorizationError),
  #[error(transparent)]
  FieldError(#[from] FieldError),
  #[error(transparent)]
  UserError(#[from] UserError),
  
  #[error(transparent)]
  AoSError(#[from] AoSError),
}

#[derive(Error, Serialize, Debug, TS)]
pub enum AoSError {
  #[error(transparent)]
  S(#[from] ServerError),
  #[error(transparent)]
  A(#[from] AuthenticationError),
}

