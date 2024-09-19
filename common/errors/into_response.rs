use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use super::*;
use super::merged::*;

impl IntoResponse for ApiError {
  fn into_response(self) -> Response {
    let mut res = Json(self).into_response();
    *res.status_mut() = StatusCode::BAD_REQUEST;
    res
  }
}

impl IntoResponse for ServerError {
  fn into_response(self) -> Response { ApiError::from(self).into_response() }
}
impl IntoResponse for AuthenticationError {
  fn into_response(self) -> Response { ApiError::from(self).into_response() }
}
impl IntoResponse for AuthorizationError {
  fn into_response(self) -> Response { ApiError::from(self).into_response() }
}
impl IntoResponse for FieldError {
  fn into_response(self) -> Response { ApiError::from(self).into_response() }
}
impl IntoResponse for UserError {
  fn into_response(self) -> Response { ApiError::from(self).into_response() }
}
impl IntoResponse for AoSeError {
  fn into_response(self) -> Response { ApiError::from(self).into_response() }
}
impl IntoResponse for AeSeError {
  fn into_response(self) -> Response { ApiError::from(self).into_response() }
}
impl IntoResponse for FiSeError {
  fn into_response(self) -> Response { ApiError::from(self).into_response() }
}
impl IntoResponse for UsSeError {
  fn into_response(self) -> Response { ApiError::from(self).into_response() }
}



