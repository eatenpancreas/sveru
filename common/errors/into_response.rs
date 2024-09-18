use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use crate::common::errors::*;

impl IntoResponse for ApiError {
  fn into_response(self) -> Response {
    let res = Json(self).into_response();
    res.status() = StatusCode::BAD_REQUEST;
    res
  }
}

impl IntoResponse for ServerError {
  fn into_response(self) -> Response {
    ApiError::from(self).into_response()
  }
}

impl IntoResponse for AuthenticationError {
  fn into_response(self) -> Response {
    ApiError::from(self).into_response()
  }
}

impl IntoResponse for AuthorizationError {
  fn into_response(self) -> Response {
    ApiError::from(self).into_response()
  }
}

impl IntoResponse for FieldError {
  fn into_response(self) -> Response {
    ApiError::from(self).into_response()
  }
}

impl IntoResponse for UserError {
  fn into_response(self) -> Response {
    ApiError::from(self).into_response()
  }
}