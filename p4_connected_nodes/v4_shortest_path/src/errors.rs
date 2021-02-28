#[derive(Debug)]
pub struct GraphErr{
    mess: String,
}

// imp for graph err
impl GraphErr {
    pub fn new(s: &str) -> Self {
        GraphErr {
            mess: s.to_string(),
        }
    }
}


// tests for err
#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  pub fn test_create_err(){
    let err = GraphErr::new("some new error on graphs");
    assert_eq!(err.mess, "some new error on graphs");
  }
}