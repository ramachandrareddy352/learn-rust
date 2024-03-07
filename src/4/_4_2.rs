use std::mem::size_of_val;

fn main() {
    // 1
    // char follows unicode values
    let c1: char = 'a';
    println!("{}", size_of_val(&c1)); // each char has 4 bytes of size in memory
    let c2 = '@';
    println!("{}", size_of_val(&c2));

    // 2
    let c1: char = 'r'; // single quotes for char
    print_char(c1); // double quotes for string

    // 3
    let t = false;
    if !t || t {
        println!("success");
    }

    // 4
    let _v: () = ();
    let v = (2, 3);
    assert_eq!(_v, implicitly_ret_unit());

    println!("Success!");

    // 6
    let unit: () = ();
    assert!(size_of_val(&unit) == 0);
    // char - 4 bytes
    // bool - 1 byte
}

fn implicitly_ret_unit() {
    println!("I will return a ()");
}

// Don't use this one
fn explicitly_ret_unit() -> () {
    println!("I will return a ()");
}

fn print_char(c: char) {
    println!("{}", c);
}
