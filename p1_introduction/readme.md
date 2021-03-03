 Covered topic includes:


 - How to upload a crate:
   - We must add to Cargo.toml all required information, to help people to find our crate.
   ```Rust
    name = "example_crate_do_not_use_uwu"
    version = "0.1.0"
    authors = ["Alejandro Vásquez Madrid <jose5m45@gmail.com>"]
    edition = "2018"
    keywords = ["do", "not", "use"]
    description = "my crate for testing purpose, do not use uwu"
    readme = "readme.md"
    license = "MIT"

   ```

 - Loops
   - usage of for loop on Rust
   - usage of custom stepper
   - while loop

 - Optional library and its usage
   - how to use standard __Result__ api of Rust.

 - Structs
   - How to declare a struct
   - How to create a enum, and assign on a property of a struct

    ```Rust
    #[derive(Debug)]
    pub struct Person {
        name: String,
        age: i32,
        children: i32,
        favorite_color: Color,
    }
    ```

   - How to use match with the enum, to evaluate different sections of code by __Color__ enum.

   ```Rust
    #[derive(Debug)]
    pub enum Color{
        Red(String),
        Green,
        Blue
    }

    fn main() {

      let color = Color::Red("im the param of Red enum".to_string());
      match color {
          Color::Red(s) => println!("Soy Rojito, {}", s),
          Color::Green => println!("Soy Verdesito"),
          Color::Blue => println!("Soy Azulito"),
      }
    }
   ```

 - Pointers
   - How type of borrow we use, alters the flow of application.
   - &self -> borrow never change ownership.
   - &mut self -> mutable borrow, we can change anything, and still using it on parent function with any change made to it.
   - self -> not borrowed value will be destroyed after first use. Also will never accesible to parent function call.
   - mut self -> same as not borrowed, but it can change on new owner of memory.

 - Variables
   - on Box section, we use Box to create a variable that will have a pointer to a list that can grows infinite *almost if we have enought memory.