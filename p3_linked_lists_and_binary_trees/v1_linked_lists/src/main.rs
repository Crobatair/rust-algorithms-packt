
/// create a struct linkedlist
#[derive(Debug)]
pub struct LinkedList<T> (Option<(T, Box<LinkedList<T>>)>);

use std::fmt::Debug;
impl <T: Debug> LinkedList<T> {

    /// impl of create a new obejct of LinkedList
    pub fn new() -> Self {
        LinkedList(None)
    }

    /// push a element on first index of LinkedList
    pub fn push_front(&mut self, data: T) {
        
        let t = self.0.take();
        self.0 = Some((data, Box::new(LinkedList(t))));
            
    }
    
    pub fn push_back(&mut self, data: T) {
        match self.0 {
            Some((_, ref mut child)) => child.push_back(data),
            None => self.push_front(data),
        }
    }

    /// Added param, to pass a index to insert new object.
    /// param is a Optional, if user does not define a number
    /// the new LinkedList will be inserted at end of LinkedList
    pub fn push_into(&mut self, data: T, index: Option<i32>) {
        match self.0 {
            // decoupled as (<T, LinkedList<T>>) because is a tuple
            Some((_, ref mut child)) => {
                match index {
                    Some(i) => {
                        /// if index is 0, push to front
                        if i == 0 {
                            self.push_front(data)
                        } else if i >= 1 {
                            // If index is 1 or higher, find parent LinkedList and insert into it.
                            child.push_into(data, Some(i-1))
                        }
                    },
                    _ => self.push_back(data),
                }
            },
            None => self.push_front(data),
        }
    }
}



fn main() {

    let mut ll = LinkedList::new();
    ll.push_front(3);
    ll.push_back(12);
    ll.push_front(1);
    ll.push_into(10, Some(2));
    println!("ll = {:?}", ll);

}


