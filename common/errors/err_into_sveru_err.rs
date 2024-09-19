
use crate::common::errors::ServerError;

pub trait IntoServerError{
  fn into_server_error(self, code: &'static str) -> ServerError;
}
pub trait ResultIntoServerError<T, E: IntoServerError>{
  fn into_server_error(self, code: &'static str) -> Result<T, ServerError>;
}

impl IntoServerError for sqlx::Error {
  fn into_server_error(self, code: &'static str) -> ServerError {
    ServerError::new_string(format!("Something went wrong with the database: {}", self.to_string()), code)
  }
}

impl <T> ResultIntoServerError<T, sqlx::Error> for Result<T, sqlx::Error> {
  fn into_server_error(self, code: &'static str) -> Result<T, ServerError> {
    self.map_err(|x| x.into_server_error(code))
  }
}