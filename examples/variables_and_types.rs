#![allow(dead_code)]

#[derive(Debug)]
struct MyStruct {
    field1: bool,
    field2: Vec<u32>,
}

#[derive(Debug)]
enum MyEnum {
    SomeVariant,
    MightHoldAnInt(Option<u8>),
}

fn main() {
    let mut flip_flop = false;
    flip_flop = !flip_flop;
    println!("`flip_flip`: {flip_flop}");

    let my_struct = MyStruct {
        field1: true,
        field2: vec![1, 2, 3],
    };
    println!("`my_struct`: {my_struct:#?}");

    let my_enum = MyEnum::MightHoldAnInt(Some(10));
    println!("`my_enum`: {my_enum:#?}");
}
