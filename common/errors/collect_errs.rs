pub trait CollectFirstErr<T> {
  fn collect_errs(self) -> T;
}

impl <T, E> CollectFirstErr<Result<Vec<T>, E>> for Vec<Result<T, E>> {
  fn collect_errs(self) -> Result<Vec<T>, E> {
    self.into_iter().collect()
  }
}
