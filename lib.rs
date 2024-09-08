pub mod macro_functions {
  mod endpoint;
  pub use endpoint::*;
}
pub mod common {
  pub mod errors;
}

pub use sveru_macro::endpoint;
use axum::response::Result;

pub type AxumResult<T> = Result<T>;
