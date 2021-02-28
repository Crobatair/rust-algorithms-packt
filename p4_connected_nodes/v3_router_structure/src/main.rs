mod errors;
use std::rc::Rc;
use std::fmt;

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

// added a struct Route, that will hold, pos, a path with a Ref Counter to a Route ID and a len
#[derive(Debug)]
pub struct Route<ID>{
    pos: ID,
    path: Option<Rc<Route<ID>>>,
    len: i32,
}

// Impl of Route, to wrap rc 
impl <ID: Eq> Route<ID> {
    pub fn start_rc(pos: ID) -> Rc<Self> {
        Rc::new(Route {
            pos,
            path: None,
            len: 0,
        })
    }

    // impl of contains for Route<ID>, return a bool with result
    pub fn contains(&self, id: &ID) -> bool {

        // current pos eqls to compared id
        if self.pos == *id {
            return true;
        }

        // if path contains on some place the id, then will return the result
        match self.path {
            Some(ref p) => p.contains(id),
            None=> false,
        }
    }
}

// formatter Display for Route<ID>
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
