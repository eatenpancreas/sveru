use serde::Serialize;
use thiserror::Error;
use ts_rs::TS;
use super::*;

#[derive(Error, Serialize, Debug, TS)]
#[serde(untagged)]
pub enum ApiError {
  #[error(transparent)]
  FieldError(#[from] FieldError),
  #[error(transparent)]
  UserError(#[from] UserError),
  #[error(transparent)]
  AuthenticationError(#[from] AuthenticationError),
  #[error(transparent)]
  AuthorizationError(#[from] AuthorizationError),
  #[error(transparent)]
  ServerError(#[from] ServerError),
}

#[derive(Error, Serialize, Debug, TS)]
#[serde(untagged)]
pub enum AeSeError {
  #[error(transparent)]
  Ae(#[from] AuthenticationError),
  #[error(transparent)]
  Se(#[from] ServerError),
}


#[derive(Error, Serialize, Debug, TS)]
#[serde(untagged)]
pub enum AoSeError {
  #[error(transparent)]
  Ao(#[from] AuthorizationError),
  #[error(transparent)]
  Se(#[from] ServerError),
}

#[derive(Error, Serialize, Debug, TS)]
#[serde(untagged)]
pub enum FiSeError {
  #[error(transparent)]
  Fi(#[from] FieldError),
  #[error(transparent)]
  Se(#[from] ServerError),
}

#[derive(Error, Serialize, Debug, TS)]
#[serde(untagged)]
pub enum UsSeError {
  #[error(transparent)]
  Us(#[from] UserError),
  #[error(transparent)]
  Se(#[from] ServerError),
}

