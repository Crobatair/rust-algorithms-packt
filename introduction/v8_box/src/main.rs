#[derive(Debug)]
pub struct LinkedList<T>{
    data: T,
    // Option, to declare next as Optional, that can contains Some or None
    // The Some will be of type -> Box (it will be a reference location *not fixed memory lenght)
    // the box will be of type LinkedList that can be of type T
    next: Option<Box<LinkedList<T>>>,
}


// If LinkedList of Type T -> implements :std::ops::AddAssign, it will be overriten
// because i32 in ex, contains it, it can work
impl <T:std::ops::AddAssign> LinkedList<T>{

    // &mut to get borrow and can update it own state
    // n as dont care after used memory
    pub fn add_up(&mut self, n: T) {
        self.data += n;
    }

}


fn main() {
    // Must be declared as mut, because add_up mutate its state
    let mut ll = LinkedList{data:3, next:Some(Box::new(LinkedList{data:2, next:None}))};
    if let Some(ref mut v ) = ll.next{
        v.add_up(10);
    }
    println!("Hello, {:?}", ll);


    // Create mutable vector with capaciity
    let mut v: Vec<String> = Vec::new();
    v.push("hello".to_string());
    println!("the lenght of vec is {} and capacity is {}", v.len(), v.capacity());
    
    let mut v2: Vec<String> = Vec::with_capacity(100);
    v2.push("hello".to_string());
    for i in 0..105 {
        v.push(i.to_string());
    }

    println!("the lenght of vec 2 is {} and capacity is {}", v2.len(), v2.capacity());
    println!("the lenght of vec is {} and capacity is {}", v.len(), v.capacity());
	

}
