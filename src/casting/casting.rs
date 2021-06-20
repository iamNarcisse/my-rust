// Suppress all warnings from casts which overflow.
#![allow(overflowing_literals)]

pub fn index() {
    let decimal = 65.2311_f32;

    // let integer: u8 = decimal;

    let integer = decimal as u8;

    let character = integer as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    // 1000 already fits in a u16
    println!("1000 as a u16 is: {}", 1000 as u16);

    println!("1000 as a u8 is : {}", 1000 as u8);

    // For positive numbers, this is the same as the modulus
    println!("1000 mod 256 is : {}", 1000 % 256);
}
