// create a trait to use on objects
pub trait Weighted{
  fn weight(&self) ->i32;
}

// implement trait for any i32 object
impl Weighted for i32 {

  // named as weight will return a reference to a i32
  fn weight(&self) ->i32 {
      *self
  }
}
