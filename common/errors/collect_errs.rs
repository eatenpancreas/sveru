pub trait CollectFirstErr<T> {
  fn collect_errs(self) -> T;
}

impl <T, E> CollectFirstErr<Result<Vec<T>, E>> for Vec<Result<T, E>> {
  fn collect_errs(self) -> Result<Vec<T>, E> {
    let capacity = self.capacity();
    self.into_iter().try_fold(Vec::with_capacity(capacity), |mut acc, res|
      res.map(|td| {acc.push(td); acc}))
  }
}
