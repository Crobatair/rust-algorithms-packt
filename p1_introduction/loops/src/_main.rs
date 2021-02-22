


// basic example
fn main() {
    let mut n = 0;
    loop {
        n += 1;
        if n > 10 {
            break;        
        }
        println!("Counting... {}", n);
    
    }
    println!("Done");
}
