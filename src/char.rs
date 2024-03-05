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
}

fn print_char(c: char) {
    println!("{}", c);
}
