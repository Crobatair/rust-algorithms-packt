#[derive(Debug)]
pub struct Person {
    name: String,
    age: i32,
    children: i32,
    favorite_color: Color,
}


#[derive(Debug)]
pub enum Color{
    Red(String),
    Green,
    Blue
}


impl Person {
    pub fn print(self)->String{
        return format!("name = {}, age = {} has {} children", self.name, self.age, self.children)
    }

}


fn main() {
    let person = Person{
        name:"matt".to_string(),
        age:35,
        children:4,
        favorite_color: Color::Blue
    };


    //println!("Hello, people!, from {:?}", person.print());
    /* Print with rerive with Person representation  */
    println!("Hello, people!, from {:?}", person);


    let color = Color::Red("im the param of Red enum".to_string());
    match color {
        Color::Red(s) => println!("Soy Rojito, {}", s),
        Color::Green => println!("Soy Verdesito"),
        Color::Blue => println!("Soy Azulito"),
    }


}
