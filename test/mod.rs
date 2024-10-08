use crate::common::errors::{ErrorDrain, FieldError};

#[test]
pub fn test_err_conversion() {
  let mut drain = ErrorDrain::<FieldError>::new();
  let i = drain.push::<FieldError>(Ok(10)).unwrap();
  let i2 = drain.push::<()>(Err(FieldError::new("aa", "bb")));
  let drain = drain.collect().unwrap_err();

  assert_eq!(i, 10);
  assert_eq!(i2, None);
  assert_eq!(drain.len(), 1)
}
