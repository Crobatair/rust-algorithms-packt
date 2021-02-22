
#[derive(Debug)]
pub enum Res<T,E>{
    Thing(T),
    Error(E),
}


fn main() {
    let a = divide(10,5);
    let b = divide(10,0);
    println!("a = {:?}, b = {:?}", a, b);

    if let Res::Thing(v) = a {
        println!("val = {}", v)
    }


    match a {
        Res::Thing(v) => println!("val = {}", v),
        _ => {}
    }


}


fn divide(a:i32, b:i32)->Res<i32, String>{
    if b == 0 {
        return Res::Error("cannot Divide by 0".to_string());
    }
    Res::Thing(a / b)
}
