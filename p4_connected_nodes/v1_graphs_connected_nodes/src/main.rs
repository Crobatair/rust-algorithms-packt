use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::{Rc, Weak};

type Rcc<T> = Rc<RefCell<T>>;
pub fn rcc<T>(t: T) ->Rcc<T>{
    Rc::new(RefCell::new(t))
}

// edgelist
pub struct EdgelistGraph<E, ID>{
    // Data is stored ad edges at E dont care too much about the nodes.
    // Simple, slow travesal
    v: Vec<(E, ID, ID)>,
}

pub struct RccGraph<T, E> {
    node: Vec<Rcc<RccNode<T, E>>>,
}

// Weak pointer safely fail if node has been removed
// can stick edge data if neede 
pub struct RccNode<T, E> {
    data: T,
    edges: Vec<(E, Weak<RefCell<RccNode<T, E>>>)>,
}


// map based
pub struct MapGraph<T, E, ID: Hash>{
    mp: HashMap<ID, T>,
    edges: Vec<(E, ID, ID)>,
}

// mappointer based
pub struct MapPGraph<T, E, ID: Hash+Eq>{
    data: HashMap<ID,(T, Vec<ID>)>,
    edges: HashMap<ID, (E, ID, ID)>,
}


fn main() {
    println!("Hello, world!");
}
