
use std::cell::RefCell;
use std::fmt::{Debug,Write};
use std::rc::Rc;
use std::fmt;

type Rcc<T> = Rc<RefCell<T>>;
pub fn rcc<T>(t:T)-> Rcc<T>{
    Rc::new(RefCell::new(t))
}


#[derive(Debug)]
pub struct SkipNode<T: PartialOrd> {
    right: Option<Rcc<SkipNode<T>>>,
    down: Option<Rcc<SkipNode<T>>>,
    data: Rcc<T>,
}


#[derive(Debug)]
pub struct SkipList<T: PartialOrd> ( Vec<SkipNode<T>>);


// impl of SkipNode
impl <T: PartialOrd> SkipNode<T> {

    pub fn new( t: T ) -> Self {
        SkipNode {
            right: None,
            data: rcc(t),
            down: None,
        }
    }

    pub fn insert(&mut self, dt: T) -> Option<Rcc<SkipNode<T>>> {
        // bigger than right the go right
        // if right = 10 and dt = 11, then insert right
        if let Some(ref mut right) = self.right {
            // borrow to get the value until the ref exists
            // right is Rc::RefCell, so needs to be borrowed
            // * is used to take the reference
            if dt > *right.borrow().data.borrow() {
                // insert after right...
                return right.borrow_mut().insert(dt);
            }
        }

        // has lower children try them
        // because dt is not higher than right, then we need to iterate over registries, and push where needed
        if let Some(ref dw) = self.down {
            // as borrow mut to actually change if needed
            return match dw.borrow_mut().insert(dt) {

                // if child exists *down side
                Some(child) => match rand::random::<bool>(){
                    true => {
                        let dt = child.borrow().data.clone();
                        let nn = SkipNode{
                            right: self.right.take(),
                            data: dt,
                            down: Some(child),
                        };
                        let res = rcc(nn);
                        self.right = Some(res.clone());
                        Some(res)
                    },
                    false => None,
                },
                None => None,
            }
        }

        // should be before right, at bottom of node
        let mut nn = SkipNode::new(dt);
        nn.right = self.right.take(); // new right will be previous right
        let res = rcc(nn); // res must be of rcc to set on self.right *new right
        self.right = Some(res.clone()); // set as cloned ref
        Some(res) // return res
    }
}


impl <T: PartialOrd> SkipList<T> {
    
    // creates a instance of SkipList Vec
    pub fn new() -> Self {
        SkipList(Vec::new())
    }

    // inset into vec
    pub fn insert(&mut self, data: T) {

        // if no elements present, the insert at top with push
        if self.0.len() == 0 {
            self.0.push(SkipNode::new(data));
            return;
        }

        // vec will have the lowest row, with the lowest number, 
        // we need to try and insert in the highst acailable row
        for i in (0..self.0.len()).rev() {
            if data > *self.0[i].data.borrow(){

                // if i can insert into vec[i], and returns something, then
                if let Some(child) = self.0[i].insert(data) {
                    // Todo loop up to 5050 chance
                    // because behaviour is, if data is present, create a upper layer, then follows:
                    self.loop_up(child, i+1);

                }
                return;
            }


        }

        // if the element dont have a lower data object, and its higher than previous, element must be replaced at first
        let mut nn = SkipNode::new(data);

        // compiler does not like assigment of new, so is used a memory swap
        // so, creates a new SkipNode reference with data of data, and swap with original first element of Vec
        std::mem::swap(&mut nn, &mut self.0[0]);
        let res = rcc(nn);
        self.0[0].right = Some(res.clone());
        self.loop_up(res, 1);
    }


    pub fn loop_up(&mut self, ch:Rcc<SkipNode<T>>, n: usize) {

        if rand::random::<bool>() == true {
            return;
        }

        let dt = ch.borrow().data.clone();
        let mut nn = SkipNode {
            right: None,
            down: Some(ch),
            data: dt,
        };

        // if number of rows is higher or equals to Vec.len() then, upper another level 
        if n >= self.0.len(){
            self.0.push(nn);
            return;
        }

        // if its last level, then swap prev right with next right
        std::mem::swap(&mut nn, &mut self.0[n]);
        let res = rcc(nn);
        self.0[n].right = Some(res.clone());
        self.loop_up(res, n+1);
    }

}


impl <T: Debug + PartialOrd> SkipNode<T> {
    // impl of Write W, with a pointer that holds writes data
    // return a Result object *Ok()
    pub fn print_row<W:Write>(&self, w:&mut W) -> std::fmt::Result {
        // check if borrows exists
        // add {:?} to the write holder
        write!(w, "{:?}", self.data.borrow())?;
        if let Some(ref r) = self.right {
            // if right existsm then add *,* separator to writer
            write!(w, ",")?;
            // then recursive print row
            r.borrow().print_row(w)?;
        }
        // if all is ok, then return with Result::Ok
        Ok(())
    }
}

impl <T: Debug + PartialOrd> fmt::Display for SkipList<T> {
    // skiplists impl of display
    // override frm::Formatter with static ref, return a fmt::Result
    fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result {
        // if len is 0, the directly write empty list
        if self.0.len() == 0 {
            return write!(f, "Skiplists<Empty>");
        }

        // if notm then SkipNode,print_row
        // this is to handle print of rows by order
        for sn in &self.0 {
            // add to formatter, a jump line, to difference between rows
            write!(f, "\n")?;
            // print row f ( node )
            sn.print_row(f)?;
        }

        Ok(())
    }
}

fn main() {
    let mut s = SkipList::new();
    s.insert(4);
    s.insert(6);
    s.insert(8);
    s.insert(99);
    // because Display is override, then can be displayed as println!("{}", s);
    println!("s = {}", s);




}
