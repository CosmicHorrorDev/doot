#[derive(Debug)]
enum Item {
    Fizz,
    Buzz,
    FizzBuzz,
    Other(u32),
}

impl Item {
    fn new(num: u32) -> Item {
        match (num % 3 == 0, num % 5 == 0) {
            (true, false) => Item::Fizz,
            (false, true) => Item::Buzz,
            (true, true) => Item::FizzBuzz,
            _ => Item::Other(num),
        }
    }
}

fn main() {
    for i in 0..10 {
        let item = Item::new(i);
        println!("{i} - {item:?}");
    }
}
