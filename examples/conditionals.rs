use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse the second arg as a string (First is the executable path)
    let mut args = env::args();
    let _ = args.next();
    let arg: u32 = match args.next() {
        Some(arg) => arg.parse()?,
        None => {
            return Err("Requires a number to be passed".into());
        }
    };

    let msg = if arg % 2 == 0 { "Even!" } else { "Odd!" };
    println!("{msg}");

    Ok(())
}
