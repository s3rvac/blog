// First version of our Parity trait.
//
// $ rustc main1.rs -o prog
// $ ./prog

trait Parity {
    fn is_even(&self) -> bool;
    fn is_odd(&self) -> bool;
}

impl Parity for i32 {
    fn is_even(&self) -> bool {
        self % 2 == 0
    }

    fn is_odd(&self) -> bool {
        !self.is_even()
    }
}

fn main() {
    let n = 20;
    if n.is_even() {
        println!("{} is even", n);
    }

    println!("{}", 123.is_even());
}
