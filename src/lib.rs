/// another object can be instantiated from this one.
/// Similar to clone except that the object do not require to got all match field (only be similar
/// to what it would be at initialization : inner state do not requires to be clone).
pub trait Proto {
  fn get_new(&self) -> Self;
}

impl Proto for () {
  #[inline]
  fn get_new(&self) -> Self {
    ()
  }
}
