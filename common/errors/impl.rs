use crate::common::errors::*;

impl ServerError {
  pub fn new(message: &str, code: &'static str) -> Self { Self::new_string(message.to_string(), code) }
  pub fn new_string(message: String, code: &'static str) -> Self { Self { message, code } }
  pub fn idc() -> Self { Self::new("Unknown server error occurred -- See context", "ERR-CODE:XX9999")}
}
impl AuthenticationError {
  pub fn new(message: &str) -> Self { Self { message: message.to_string() }}
  pub fn new_string(message: String) -> Self { Self { message } }
}
impl AuthorizationError {
  pub fn new(message: &str) -> Self { Self { message: message.to_string() }}
  pub fn new_string(message: String) -> Self { Self { message } }
}
impl FieldError {
  pub fn new(message: &str, field: &str) -> Self { Self::new_string(field.to_string(), message.to_string())}
  pub fn new_string(message: String, field: String) -> Self { Self { message, field } }
}
impl UserError {
  pub fn new(message: &str) -> Self { Self { message: message.to_string() }}
}