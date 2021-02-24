use std::fmt::Debug;

#[derive(Debug)]
pub struct BinTree<T> (Option<Box<BinData<T>>>);

#[derive(Debug)]
pub struct BinData<T> {
    data: T,
    left: BinTree<T>,
    right: BinTree<T>,
}

// impl of BinTree
impl<T> BinTree<T> {

    // create a new BinTree
    pub fn new() -> Self {
        BinTree(None)
    }
}

impl<T: PartialOrd> BinTree<T>{
    pub fn add_sorted(&mut self, data: T){
        match self.0 {
            Some(ref mut db) => {
                if data < db.data {
                    db.left.add_sorted(data);
                } else {
                    db.right.add_sorted(data);
                }
            },
            None => {
                self.0 = Some(Box::new(BinData{data: data, left: BinTree::new(), right: BinTree::new()}));
            }
        }
    }
}


impl <T: Debug> BinTree<T> {
 
    pub fn print_lfirst(&self, depth: i32){
        if let Some(ref bd) = self.0 {
            bd.left.print_lfirst(depth + 1);
            let mut spc = String::new();
            for _ in 0..depth{
                spc.push('.');
            }
            println!("{}{:?}", spc, bd.data);
            bd.right.print_lfirst(depth+1);
        }
    }

}


fn main() {

    let mut t = BinTree::new();
    t.add_sorted(4);
    t.add_sorted(5);
    t.add_sorted(1);
    t.add_sorted(2);
    t.add_sorted(3);
    t.print_lfirst(0);
    println!("{:?}", t)
}
