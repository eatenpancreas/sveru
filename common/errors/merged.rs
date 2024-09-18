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
}

#[derive(Error, Serialize, Debug, TS)]
#[serde(untagged)]
pub enum AeSeError {
  #[error(transparent)]
  S(#[from] ServerError),
  #[error(transparent)]
  A(#[from] AuthenticationError),
}


#[derive(Error, Serialize, Debug, TS)]
#[serde(untagged)]
pub enum AoSeError {
  #[error(transparent)]
  S(#[from] ServerError),
  #[error(transparent)]
  A(#[from] AuthorizationError),
}

