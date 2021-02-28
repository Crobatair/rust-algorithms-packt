mod errors;
use std::rc::Rc;
use std::collections::HashMap;
use std::hash::Hash;
use std::fmt;

pub trait Weighted{
    fn weight(&self) ->i32;
}

impl Weighted for i32 {
    fn weight(&self) ->i32 {
        *self
    }
}

#[derive(Debug)]
pub struct Route<ID>{
    pos: ID,
    path: Option<Rc<Route<ID>>>,
    len: i32,
}

impl <ID: Eq> Route<ID> {
    pub fn start_rc(pos: ID) -> Rc<Self> {
        Rc::new(Route {
            pos,
            path: None,
            len: 0,
        })
    }

    pub fn contains(&self, id: &ID) -> bool {

        if self.pos == *id {
            return true;
        }

        match self.path {
            Some(ref p) => p.contains(id),
            None=> false,
        }
    }
}

impl <ID:fmt::Debug> fmt::Display for Route<ID> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        if let Some(ref p) = self.path {
            write!(f, "{}-{}-", p, self.len)?;
        }
        write!(f, "{:?}", self.pos)
    }

}




fn main() {
    println!("Hello, world!");
}
