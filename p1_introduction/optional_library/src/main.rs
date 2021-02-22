fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
         return Result::Err("Cannot divide by 0".to_string());
    }
    return Result::Ok(a/b);
}


fn main() {
    let a = divide(10, 5);
    let b = divide(10, 0);
    
    match a {
        Result::Ok(r)=> println!("Result from optional = {}", r),
        _ => {}
    }

    if let Result::Err(r) = b {
    
        println!("Result of b = {}", r);
    }

}
