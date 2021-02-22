
// Create a structure with name and age.
// Derive Clone is added to implement, copy all data of object on new memory location on new variable
// if clone is not specified, obj will be moved to other memory location completely
#[derive(Debug, Clone)]
pub struct Person {
    name: String,
    age: i32,
}


// Example of copy clone with stuct that contains native copy (i32 implements copy by default)
// but clone must be added beacuse copy by default implement *Clone*
// if Clone derive is not called, must be impl direct on impl Point
#[derive(Debug, Clone, Copy)]
pub struct Point {
    x: i32,
    y: i32,
}

// implementation of Point
impl Point {

    // Point::new impl, return a Point object
    pub fn new(x: i32, y:i32) -> Self {
        Point{x,y}
    }

}


fn main() {
    // -> Example copy of i32 that implements by  default clone
    let mut x = 32;
    let y = x;
    x = 5;
    println!("y = {} and x = {}", y, x);



    // Example of Person that had implemented Clone derive and can ve copied to new variable
    let mut person1 = Person {
        name: "Pedro".to_string(),
        age: 32,
    };
    
    // But, clone must be explicit called
    let person2 = person1.clone();
    person1.name.push_str("Harley");
    println!("person 1 is :{:?}, and person 2 is {:?}",person1, person2);


    // Point contains i32 with copy, but it need to use Clone to copy values to new location...
    // new constructor is called.
    let mut point = Point::new(2, 3);
    let point_copy = point;
    point.x = 15;
    println!("point is :{:?}, and point 2 {:?}", point, point_copy);



}
