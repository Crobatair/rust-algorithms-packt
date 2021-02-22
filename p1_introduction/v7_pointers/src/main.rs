#[derive(Debug, Clone)]
pub struct Person {
    name: String,
    age: i32
}

impl Person {
    pub fn new ( name: String, age: i32 ) -> Self {
        Person { name, age }
    }
    
    // Options below of params definition
    //pub fn print(self, &self, &mut self, mut self){}
    /**
          * self       -> change owner of param, it will never be accesible by parent...
                             let a = 54;
                             myfunc(a) --> a will be never accesible with value 54 and borrow error....

          * &self      -> can be used anywhere inside the funtion callback, and ownership never change 
          
          * &mut self  -> any change inside the funtion will be reflected to memory of object *changed from called function

          * mut self  
    */

    pub fn greet(&self) -> String {
        format!("Hi, my name is {}", self.name)
    }

    pub fn age_up(&mut self, n: i32) {
        self.age += n
    }
    
    pub fn dropme(self) {
        
    }

}




fn main() {
    let mut p = Person::new("Alejandro".to_string(), 25);
    p.age_up(3);
    //p.dropme();
    let s = p.greet();
    println!("{}", s);
    
    let s2 = p.greet();
    println!("xddd {}", s2);



    let a = get_age(&p);
    
    // compiler error, trying to change a borrow var that is not mut... mut declare all as *mut* or use it 
    //p.age_up(10);
    println!("person age is: {}", a);
    p.age_up(10); // Compiler is ok and its happy
}


// borros the Person,
// borrows the result
// the result will exist until person memory does not exist
pub fn get_age(s: &Person ) -> &i32 {
    &s.age
}
