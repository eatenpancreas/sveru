use crate::common::errors::ApiError;

pub struct ErrorDrain(Vec<ApiError>);

impl ErrorDrain {
  pub fn new() -> Self { Self ( vec![] )}
  pub fn push<T, E: Into<ApiError>>(&mut self, res: Result<T, E>) -> Option<T> {
    match res {
      Ok(t) => return Some(t),
      Err(e) => self.0.push(e.into())
    }
    None
  }
  pub fn drain(self) -> Result<(), Vec<ApiError>> {
    if self.0.len() > 0 {
      Err(self.0)
    } else {
      Ok(())
    }
  }
}