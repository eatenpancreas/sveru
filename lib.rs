#[cfg(test)]
mod test;

pub mod macro_functions {
  mod endpoint;
  pub use endpoint::*;
}
pub mod common {
  pub mod print_self;
  pub mod errors;
}
pub mod compat {
  pub mod sqlx;
}

pub use sveru_macro::endpoint;
use axum::response::Result;

pub type AxumResult<T> = Result<T>;
