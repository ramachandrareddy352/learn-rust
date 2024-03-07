use std::ops::{Range, RangeInclusive};
fn main() {
    // 2
    let v: u16 = 38_u8 as u16; // here 38 is in unsigned numbers and we are explictily converting
    println!("Success! {}", v);

    // 3
    let x: u32 = 5;
    assert_eq!("u32".to_string(), type_of(&x)); // .to_string() convert data type to string

    println!("{}", type_of(&x));
    println!("{}", "u32".to_string());
    println!("{}", "u32");
    println!("Success!");

    // 4
    assert_eq!(i8::MAX, 127); // ::MAX returns max value of that data type
    assert_eq!(u8::MAX, 255);
    println!("Success!");

    // 5
    let v1: u16 = 251_u16 + 8_u16;
    let v2: i16 = i16::checked_add(251, 8).unwrap(); // here i_8 is not possible because 251+8 = 260 overflow
    println!("{},{}", v1, v2);

    // 6
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111; // 1024 + 255 + 63 + 255
    assert!(v == 1597);
    println!("Success!");

    // 7
    let x: f64 = 1_000.000_1; // floating points
    let y: f32 = 0.12; // f32
    let z: f64 = 0.01_f64; // f64

    assert_eq!(type_of(&x), "f64".to_string());
    println!("Success!");

    // 8
    assert!(0.1_f32 + 0.2_f32 == 0.3 as f32); // we can use either (as) keyword or _f32
    println!("Success!");

    // 9
    let mut sum = 0;
    for i in -3..2 {
        // [-3, -2, -1, 0, 1]
        sum += i
    }

    assert!(sum == -5);

    for c in 'a'..='z' {
        // (=) assign z also
        println!("{}", c); // prints characters
        println!("{}", c as u8); // prints its ascii values
    }

    // 10
    assert_eq!((1..5), Range { start: 1, end: 5 }); // 5 is not included
    assert_eq!((1..=5), RangeInclusive::new(1, 5)); // 5 is included
    println!("Success!");

    // 11
    // Integer addition
    assert!(1u32 + 2 == 3);

    // Integer subtraction
    assert!(1i32 - 2 == -1);
    assert!(1i8 - 2 == -1);

    assert!(3 * 50 == 150); // default is 132

    assert!(9.6_f32 / 3.2_f32 == 3.0_f32); // error ! make it work

    assert!(24 % 5 == 4);
    // Short-circuiting boolean logic
    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101); // return in terms of binanry output
    println!("0011 OR 0101 is {}", 0b0011u32 | 0b0101); // return in i32 type
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
    println!("0x80 >> 2 is 0x{:b}", 0x80u32 >> 2);
    println!("0x80 >> 2 is 0x{:08b}", 0x80u32 >> 2); // 08 is no.of bits to reperesent the data
    println!("0x80 >> 2 is 0o{:o}", 0x80u32 >> 2);

    // :x = hexa
    // :o = octal
    // :b = binary
}

// Get the type of given variable, return a string representation of the type  , e.g "i8", "u8", "i32", "u32"
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}
