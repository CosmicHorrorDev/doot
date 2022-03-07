fn is_even(num: u32) -> bool {
    num % 2 == 0
}

struct BoolHolder {
    inner: bool,
}

impl BoolHolder {
    fn new() -> BoolHolder {
        Self { inner: false }
    }

    fn get_inner(&self) -> bool {
        self.inner
    }

    fn flip_inner(&mut self) {
        self.inner = !self.inner;
    }
}

fn main() {
    let num = 2;
    println!("{num} is even: {}", is_even(num));

    let mut bool_holder = BoolHolder::new();
    println!("`bool_holder`'s inner: {}", bool_holder.get_inner());
    bool_holder.flip_inner();
    println!("After flipping: {}", bool_holder.get_inner());
}
