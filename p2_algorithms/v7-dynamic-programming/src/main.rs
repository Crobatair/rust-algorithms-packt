use std::time::Instant;
fn main() {

    let fi_a = Instant::now();
    let a = fibonacci(17);
    let el_a = fi_a.elapsed();

    let fi_b = Instant::now();
    let b = fibonacci_iter(17);
    let el_b = fi_b.elapsed();

    let fi_c = Instant::now();
    let c = fibonacci_dynamic_tail_recursive_wrapper(17);
    let el_c = fi_c.elapsed();

    println!("normal {}, iter {}, dynamic {}", a, b, c);
    println!("a {:.2?}, b {:.2?}, c {:.2?}", el_a, el_b, el_c);
    panic!("");
}

pub fn fibonacci(n: i32) -> i32 {
    if n <= 1 {
        return 1;
    }

    fibonacci(n-1) + fibonacci(n -2)
}

pub fn fibonacci_iter(n: i32) -> i32 {
    let mut a = 1;
    let mut b = 1;
    let mut res = 1;

    for _ in 1..n {
        res = a+b;
        a = b;
        b = res;
    }
    res
}

// return (res, prev)
// if you are going to use the same funtion mote than once
// store the result somewhere
pub fn fibonacci_dynamic(n: i32) -> (i32, i32) {
    if n == 0{
        return (1,0);
    }

    let (a, b) = fibonacci_dynamic(n-1);
    (a + b, a)
}

/// wrapper to calculate fibonacci dynamic
pub fn fibonacci_dynamic_tail_recursive_wrapper(n: i32) -> i32 {
    let mut a = 1;
    let mut b = 1;

    if n <= 1 {
        return 1;
    }

    fibonacci_dynamic_tail_recursive(n, &mut a, &mut b)
}

/// recursive fibonacci algorithm with dynamic programming
/// and tail recursive function
pub fn fibonacci_dynamic_tail_recursive(n: i32, a: &mut i32, b: &mut i32) -> i32 {

    if n <= 1 {
        return *b;
    } 

    let res = *a + *b;
    *a = *b;
    *b = res;
    
    fibonacci_dynamic_tail_recursive(n-1, a, b)
}

