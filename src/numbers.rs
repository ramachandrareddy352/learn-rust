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
}

// Get the type of given variable, return a string representation of the type  , e.g "i8", "u8", "i32", "u32"
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}
