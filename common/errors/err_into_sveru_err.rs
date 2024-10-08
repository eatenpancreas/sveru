use bcrypt::BcryptError;
use sqlx::types::uuid;
use crate::common::errors::ServerError;

pub trait IntoServerError{
  fn into_server_error(self, code: &'static str) -> ServerError;
}
pub trait ResultIntoServerError<T>{
  fn into_server_error(self, code: &'static str) -> Result<T, ServerError>;
}

impl IntoServerError for sqlx::Error {
  fn into_server_error(self, code: &'static str) -> ServerError {
    ServerError::new_string(format!("Something went wrong with the database: {}", self.to_string()), code)
  }
}

impl <T> ResultIntoServerError<T> for Result<T, sqlx::Error> {
  fn into_server_error(self, code: &'static str) -> Result<T, ServerError> {
    self.map_err(|x| x.into_server_error(code))
  }
}

impl IntoServerError for jsonwebtoken::errors::Error {
  fn into_server_error(self, code: &'static str) -> ServerError {
    ServerError::new_string(format!("Something went wrong with the server parsing: {}", self.to_string()), code)
  }
}

impl <T> ResultIntoServerError<T> for Result<T, jsonwebtoken::errors::Error> {
  fn into_server_error(self, code: &'static str) -> Result<T, ServerError> {
    self.map_err(|x| x.into_server_error(code))
  }
}

impl IntoServerError for BcryptError {
  fn into_server_error(self, code: &'static str) -> ServerError {
    ServerError::new_string(format!("Something went wrong with server encryption: {}", self.to_string()), code)
  }
}

impl <T> ResultIntoServerError<T> for Result<T, BcryptError> {
  fn into_server_error(self, code: &'static str) -> Result<T, ServerError> {
    self.map_err(|x| x.into_server_error(code))
  }
}


impl IntoServerError for uuid::Error {
  fn into_server_error(self, code: &'static str) -> ServerError {
    ServerError::new_string(format!("Something went wrong with uuid parsing: {}", self.to_string()), code)
  }
}

impl <T> ResultIntoServerError<T> for Result<T, uuid::Error> {
  fn into_server_error(self, code: &'static str) -> Result<T, ServerError> {
    self.map_err(|x| x.into_server_error(code))
  }
}