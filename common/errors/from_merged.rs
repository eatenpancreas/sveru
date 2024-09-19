use axum::response::{IntoResponse, Response};
use crate::common::errors::{AeSeError, AoSeError, ApiError, FiSeError, UsSeError};

impl From<AoSeError> for ApiError {
  fn from(value: AoSeError) -> Self {
    match value {
      AoSeError::Ao(e) => e.into(),
      AoSeError::Se(e) => e.into(),
    }
  }
}
impl From<AoSeError> for AeSeError {
  fn from(value: AeSeError) -> Self {
    match value {
      AeSeError::Ae(e) => e.into(),
      AeSeError::Se(e) => e.into(),
    }
  }
}
impl From<AoSeError> for FiSeError {
  fn from(value: FiSeError) -> Self {
    match value {
      FiSeError::Fi(e) => e.into(),
      FiSeError::Se(e) => e.into(),
    }
  }
}
impl From<AoSeError> for UsSeError {
  fn from(value: UsSeError) -> Self {
    match value {
      UsSeError::Us(e) => e.into(),
      UsSeError::Se(e) => e.into(),
    }
  }
}