fn main() {
    let s = "123";
    let num: u32 = match s.parse() {
        Ok(num) => num,
        Err(_) => panic!("Not a number!"),
    };

    let msg = if num % 2 == 0 { "Even!" } else { "Odd!" };
    println!("{msg}");
}
