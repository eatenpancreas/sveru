use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorDrain<Err: Serialize>(Vec<Err>);

impl <Err: Serialize> ErrorDrain<Err> {
  pub fn new() -> Self { Self ( vec![] )}
  pub fn push<T>(&mut self, res: Result<T, Err>) -> Option<T> {
    match res {
      Ok(t) => return Some(t),
      Err(e) => self.0.push(e)
    }
    None
  }
  pub fn push_into<T, E: Into<Err>>(&mut self, res: Result<T, E>) -> Option<T> {
    match res {
      Ok(t) => return Some(t),
      Err(e) => self.0.push(e.into())
    }
    None
  }
  pub fn enforce_all(self) -> Result<(), Self> {
    if self.0.len() > 0 { Err(self) } else { Ok(()) }
  }
}