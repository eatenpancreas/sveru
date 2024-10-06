pub trait CollectFirstErr<T> {
  fn collect_first_err(self) -> T;
}

impl <T, E> CollectFirstErr<Result<Vec<T>, E>> for Vec<Result<T, E>> {
  fn collect_first_err(self) -> Result<Vec<T>, E> {
    self.into_iter().collect()
  }
}
