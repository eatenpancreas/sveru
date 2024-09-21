use std::fmt::Debug;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct ErrorDrain<Err: Serialize + Debug>(pub(crate) Vec<Err>);
pub struct ErrorDrainWith<T, Err: Serialize + Debug>(ErrorDrain<Err>, Option<T>);

impl <Err: Serialize + Debug> From<Err> for ErrorDrain<Err> {
  fn from(value: Err) -> Self {
    let mut drain = ErrorDrain::new();
    drain.push_err(value);
    drain
  }
}

impl <Err: Serialize + Debug> ErrorDrain<Err> {
  pub fn new() -> Self { Self ( vec![] )}
  pub fn push_err(&mut self, err: Err) {
    self.0.push(err);
  }
  pub fn push<T>(&mut self, res: Result<T, Err>) -> Option<T> {
    match res {
      Ok(t) => return Some(t),
      Err(e) => { self.0.push(e); None }
    }
  }
  pub fn push_into<T, E: Into<Err>>(&mut self, res: Result<T, E>) -> Option<T> {
    self.push(res.map_err(|e| e.into()))
  }
  pub fn with<T>(mut self, res: Result<T, Err>) -> ErrorDrainWith<T, Err> {
    let t = self.push(res);
    ErrorDrainWith(self, t)
  }
  pub fn with_into<T, E: Into<Err>>(self, res: Result<T, E>) -> ErrorDrainWith<T, Err> {
    self.with(res.map_err(|e| e.into()))
  }
  pub fn with_err(mut self, err: Err) -> Self {
    self.push_err(err);
    self
  }
  pub fn flush(self) -> Result<(), Self> {
    if self.0.len() > 0 { Err(self) } else { Ok(()) }
  }
}

impl <T, Err: Serialize + Debug> ErrorDrainWith<T, Err> {
  pub fn with<T2>(mut self, res: Result<T2, Err>) -> ErrorDrainWith<(T, T2), Err> {
    let mut tt = None;
    if let Some(t2) = self.0.push(res) {
      if let Some(t) = self.1 {
        tt = Some((t, t2))
      }
    }
    ErrorDrainWith(self.0, tt)
  }
  pub fn with_into<T2, E: Into<Err>>(self, res: Result<T2, E>) -> ErrorDrainWith<(T, T2), Err> {
    self.with(res.map_err(|e| e.into()))
  }
  pub fn with_err(mut self, err: Err) -> Self {
    self.0.push_err(err);
    self
  }
  pub fn flush(self) -> Result<T, ErrorDrain<Err>> {
    self.0.flush().map(|_| self.1.unwrap() )
  }
}