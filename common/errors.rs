use std::cmp::PartialEq;
use std::error::Error;
use std::fmt::Debug;
use askama_axum::Response;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use derived_deref::{Deref, DerefMut};
use serde::Serialize;

#[derive(Debug, Deref, DerefMut, Clone)]
pub struct ViewErrors(Vec<ViewError>);

#[derive(Debug, Serialize, Clone)]
pub struct ViewError {
  pub message: String,
  #[serde(flatten)]
  pub cause: Cause
}

impl ViewError {
  pub fn new(message: &str, cause: Cause) -> Self {
    Self { message: message.to_string(), cause }
  }
}

#[derive(Debug, Serialize, Clone, PartialEq)]
#[serde(tag = "cause")]
#[serde(rename_all = "lowercase")]
pub enum Cause {
  Server,
  Root { #[serde(skip_serializing)] status: StatusCode },
  Field { field: String },
  Auth,
}

impl Cause {
  pub fn field(field: &str) -> Cause { Cause::Field { field: field.to_string() }}
  pub fn root_default() -> Cause { Cause::Root { status: StatusCode::FORBIDDEN }}
  pub fn is_root(&self) -> bool { if let Cause::Root {..} = self {true} else {false} }
  pub fn is_field(&self) -> bool { if let Cause::Field {..} = self {true} else {false} }
}

pub trait CombineIntoViewErrors {
  type Output;
  fn combine(self) -> Result<Self::Output, ViewErrors>;
}

pub trait IntoViewError<T, E> {
  fn into_view(self, cause: Cause) -> Result<T, ViewError>;
}

pub trait LogIntoViewError<T, E> {
  fn log_into_view(self, cause: Cause) -> Result<T, ViewError>;
}

impl CombineIntoViewErrors for () {
  type Output = ();
  fn combine(self) -> Result<Self::Output, ViewErrors> {
    Ok(())
  }
}

impl<T> CombineIntoViewErrors for Result<T, ViewError> {
  type Output = T;
  fn combine(self) -> Result<Self::Output, ViewErrors> {
    self.map_err(|e| ViewErrors(vec![e]))
  }
}

impl<Head, Tail> CombineIntoViewErrors for (Result<Head, ViewError>, Tail)
  where
    Tail: CombineIntoViewErrors,
{
  type Output = (Head, Tail::Output);

  fn combine(self) -> Result<Self::Output, ViewErrors> {
    let (head_result, tail) = self;

    match head_result {
      Ok(head) => tail.combine().map(
        |tail| (head, tail)
      ),
      Err(head_error) => match tail.combine() {
        Ok(_) => Err(ViewErrors(vec![head_error])),
        Err(mut tail_errors) => {
          tail_errors.insert(0, head_error);
          Err(tail_errors)
        }
      },
    }
  }
}

impl <T, E: Error + 'static> IntoViewError<T, E> for Result<T, E> {
  fn into_view(self, cause: Cause) -> Result<T, ViewError> {
    Ok(self.map_err(|err| ViewError {
      message: err.to_string(),
      cause,
    })?)
  }
}

impl <T, E: Error + Debug + 'static> LogIntoViewError<T, E> for Result<T, E> {
  fn log_into_view(self, cause: Cause) -> Result<T, ViewError> {
    if let Err(e) = &self {
      eprintln!("ERROR: {e:?}");
    }
    self.into_view(cause)
  }
}

impl IntoResponse for ViewErrors {
  fn into_response(self) -> Response {
    error_response(self.0)
  }
}

impl IntoResponse for ViewError {
  fn into_response(self) -> Response {
    error_response(vec![self])
  }
}

fn error_response(errs: Vec<ViewError>) -> Response {
  let status = error_response_status_code(&errs);
  let mut res = Json(errs).into_response();

  *res.status_mut() = status;

  res
}

#[inline]
fn error_response_status_code(errs: &Vec<ViewError>) -> StatusCode {
  if errs.iter().find(|err| err.cause == Cause::Server).is_some() {
    return StatusCode::INTERNAL_SERVER_ERROR
  }

  if let Some(ViewError { cause: Cause::Root { status }, ..}) =
    errs.iter().find(|e| e.cause.is_root())
  {
    return *status
  }

  if errs.iter().find(|err| err.cause.is_field()).is_some() {
    return StatusCode::UNPROCESSABLE_ENTITY
  }

  return StatusCode::FORBIDDEN
}
