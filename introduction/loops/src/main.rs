pub struct Stepper {
    curr: i32,
    step: i32,
    max: i32,
}

impl Iterator for Stepper {

    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        if self.curr >= self.max {
            return None;
        }
        let res = self.curr;
        self.curr += self.step;
        Some(res)
    }

}


fn main() {
    let mut stepper = Stepper{ curr: 2, step: 3, max: 15 };
    loop {
        match stepper.next(){
            Some(v) => println!("loop data {}", v),
            None => break
        }
    }

    stepper = Stepper{ curr: 2, step: 3, max: 15 };
    while let Some(v) = stepper.next() {
        println!("while iterating... {}", v)
    }

    stepper = Stepper{ curr: 2, step: 3, max: 15 };
    for i in stepper {
        println!("for loop... {}", i)
    }
}
