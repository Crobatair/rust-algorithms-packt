
// Inmutable outside, but internally, can mutate
use std::cell::RefCell;

// Reference counting pointer
// Weak -> can drop even if reference exists
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct DbNode<T>{
    data: T,
    next: Option<Rc<RefCell<DbNode<T>>>>,
    prev: Option<Weak<RefCell<DbNode<T>>>>
}

#[derive(Debug)]
pub struct DdList<T> {
    first: Option<Rc<RefCell<DbNode<T>>>>,
    last: Option<Weak<RefCell<DbNode<T>>>>,
    
}

use std::fmt::Debug;
impl <T: Debug> DdList<T> {

    pub fn new() -> Self {
        DdList {
            first: None,
            last: None,
        }
    }

    pub fn push_front(&mut self, data: T) {
        match self.first.take() {
            Some(r)=>{
                let new_front = Rc::new(RefCell::new(
                    DbNode {
                        data,
                        next: Some(r.clone()),
                        prev: None,
                    }
                ));

                let mut m = r.borrow_mut();
                m.prev = Some(Rc::downgrade(&new_front));
                self.first = Some(new_front);

            },
            None=> {
                let new_data = Rc::new(RefCell::new(DbNode {
                    data,
                    next: None,
                    prev: None,
                }));

                self.last = Some(Rc::downgrade(&new_data));
                self.first = Some(new_data);
            },
        }
    }

    pub fn push_back(&mut self, data: T) {
        match self.last.take() {
            Some(r)=>{
                let new_back = Rc::new(RefCell::new(
                    DbNode {
                        data,
                        prev: Some(r.clone()),
                        next: None,
                    }
                ));

                let st = Weak::upgrade(&r).unwrap();
                let mut m = st.borrow_mut();
                self.last = Some(Rc::downgrade(&new_back));
                m.next = Some(new_back);

            },
            None=> {
                let new_data = Rc::new(RefCell::new(DbNode {
                    data,
                    next: None,
                    prev: None,
                }));
                self.last = Some(Rc::downgrade(&new_data));

                self.first = Some(new_data);
            },
        }
    }


    /// Remove form first element
    pub fn pop(&mut self) -> Option<Rc<RefCell<DbNode<T>>>> {
        match self.first.take() {
            Some(r) => {
                let next_first = r.borrow_mut().next.clone();
                match next_first {
                    Some(unwrapped_next_first) =>{
                        self.first = Some(unwrapped_next_first.clone());
                        let v = unwrapped_next_first.borrow_mut().next.clone();
                        match v {
                            Some(_) => {},
                            None => { self.last = None },
                        }
                    },
                    None => {
                        self.first = None;
                        self.last = None;
                    },
                }
                Some(r)
            },
            None => None,
        }
    }

    

    /// remove from last element
    pub fn pop_last(&mut self) -> Option<Rc<RefCell<DbNode<T>>>> {
        match self.last.take() {

            // if last exists, then will return last
            Some(last) => {

                // upgrade to return same type as pop, and can use borrow to alter .next variables
                let rc_last = Weak::upgrade(&last);

                // upgrade can return Some or None if Weak ref does not exists
                match rc_last {

                    // if exists, then set prev to last
                    Some(ref unwrapped_last) => {

                        // because is last, next must be None
                        // assign self.last with last.prev.
                        unwrapped_last.borrow_mut().next = None;
                        self.last = unwrapped_last.borrow_mut().prev.clone();

                        
                        // but to remove a dangling dependecy, i need to verify that previous of last is not first
                        // if new last == first element....
                        match self.last.take() {
                            Some(exists) => {
                                // upgrade weak to take a RC::
                                let strong_last = Weak::upgrade(&exists);
                                // Tests if first == last refs ** because pop_last if last if None, already calls pop(); method, last can be cleanedup
                                if Rc::ptr_eq(&strong_last.clone().unwrap(), &self.first.clone().take().unwrap()) {
                                    match self.first.clone().take() {
                                        // so, with a copy of first, set next and prev as None,
                                        // and assign self.first with changed of first
                                        Some(first_) => {
                                            first_.borrow_mut().next = None;
                                            first_.borrow_mut().prev = None;
                                            self.first = Some(first_);
                                        },
                                        None => {}
                                    }
                                }
                            },
                            None => { self.first = None },
                        }


                        return rc_last;
                    },
                    None => None,
                }
            },
            None => {
                return self.pop();
            }
        }
    }

}



fn main() {

    let mut dl = DdList::new();
    // Add 3 elements
    dl.push_front(3);
    dl.push_front(2);
    dl.push_front(1);

    // remove (1)
    dl.pop();

    // should return (3), and dl only remains (2)
    let the_third = dl.pop_last();

    // should print (2) only
    println!("{:?}", dl);
    println!(" pop of 3 {:?}", the_third);

    // should print (2)
    println!("{:?}", dl);

    // will print (2)
    println!("{:?}", dl.pop_last());

    // will print {first : None, last: None }
    println!("{:?}", dl);

}
