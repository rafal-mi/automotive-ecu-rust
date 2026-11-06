// One comment
use std::mem::{align_of, offset_of, size_of};

#[repr(C)]
struct Example {
    a: u8,
    b: u32,
    c: u16,
}

fn main() {
    let example = Example {
        a: 0x11,
        b: 0x22334455,
        c: 0x6677,
    };

    println!("Hello!");

    println!("Size: {}", size_of::<Example>());
    println!("Alignment: {}", align_of::<Example>());

    println!("a: {}", offset_of!(Example, a));
    println!("b: {}", offset_of!(Example, b));
    println!("c: {}", offset_of!(Example, c));

    println!("Values: {} {} {}", example.a, example.b, example.c);
}
