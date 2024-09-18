use crate::common::errors::*;

impl ServerError {
  pub fn new(message: &str, code: u16) -> Self { Self { message: message.to_string(), code }}
}
impl AuthenticationError {
  pub fn new(message: &str) -> Self { Self { message: message.to_string() }}
}
impl AuthorizationError {
  pub fn new(message: &str) -> Self { Self { message: message.to_string() }}
}
impl FieldError {
  pub fn new(message: &str, field: &str) -> Self { Self { field: field.to_string(), message: message.to_string() }}
}
impl UserError {
  pub fn new(message: &str) -> Self { Self { message: message.to_string() }}
}