// Second version of our Parity trait, where is_odd() is specified in the trait
// as a default method.
//
// $ rustc main2.rs -o prog
// $ ./prog

trait Parity {
    fn is_even(&self) -> bool;

    fn is_odd(&self) -> bool {
        !self.is_even()
    }
}

impl Parity for i32 {
    fn is_even(&self) -> bool {
        self % 2 == 0
    }
}

fn main() {
    let n = 20;
    if n.is_even() {
        println!("{} is even", n);
    }

    println!("{}", 123.is_even());
}
