use sqlx::Error::*;
use crate::common::errors::ServerError;

pub trait IntoServerError{
  fn into_server_error(self, code: &'static str) -> ServerError;
}

impl IntoServerError for sqlx::Error {
  fn into_server_error(self, code: &'static str) -> ServerError {
    ServerError::new("Something went wrong with the database", code, Some(self.to_string()))
  }
}