fn main() {
    let mut flip_flop = false;
    flip_flop = !flip_flop;
    println!("`flip_flip`: {flip_flop}");

    let immutable = 1;
    // immutable += 1; <- Can't compile
}
