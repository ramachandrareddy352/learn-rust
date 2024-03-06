use std::mem::utf8_slice;
fn main() {
    // example
    let s = String::from("hello world");
    let _hello = &s[0..6];
    let _world = &s[7..11];
    println!("{}", s);
    println!("{}", &s);
    println!("{:p}", &s); // reads pointer value

    // 1
    let s: &str = "hello, world";
    let _hello = &s[0..6];
    println!("{}", s);

    // 2
    let s: Box<str> = "hello, world".into(); // using into() we can convert the str to Box<str>
    greetings(&s);

    // 3
    let mut s = String::from("");
    s.push_str("hello, world"); // push_str -> to push string
    s.push('!'); // push -> to push character
    assert_eq!(s, "hello, world!");

    // 4
    let mut s = String::from("hello");
    s.push(',');
    s.push_str(" world");
    s += "!"; // append the string
    println!("{}", s);

    // 5
    let mut s = String::from("I like dogs");
    // Allocate new memory and store the modified string there
    let s1 = s.replace("dogs", "cats");
    assert_eq!(s1, "I like cats");

    s = s.replace("dogs", "cats");
    assert_eq!(s, "I like cats");

    // 6
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    let s3 = s1.clone() + &s2;
    let s3 = s1.clone() + s2.as_str();
    assert_eq!(s3, "hello,world!");
    println!("{}", s1);

    // 7
    let s = "hello";
    greeting(s.to_string());
    greeting(String::from(s));
    let s = String::from("hello, world");
    greeting(s);

    // 8
    let s = "hello, world".to_string();
    let s1: &str = s.as_str();
    let s1: &str = &s;

    // 9
    let byte_escape = "I'm writing Ru\x73\x74!"; // \x73\x74 = st
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape); // \\ for esscape character

    // ...Or Unicode code points.
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    println!(
        "Unicode character {} (U+211D) is called {}",
        unicode_codepoint, character_name
    );

    let long_string = "String literals
                        can span multiple lines.
                        The linebreak and indentation here \
                         can be escaped too!";
    println!("{}", long_string);

    // 10
    let raw_str = r"Escapes don't work here: \x3F \u{211D}";
    // Modify above line to make it work
    assert_eq!(raw_str, "Escapes don't work here: ? ℝ");

    // If you need quotes in a raw string, add a pair of #s
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    // If you need "# in your string, just use more #s in the delimiter.
    // You can use up to 65535 #s.
    let delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", delimiter);

    // Fill the blank
    // let long_delimiter = ;
    // assert_eq!(long_delimiter, "Hello, \"##\"");

    println!("Success!");

    // 11
    let s1 = String::from("hi,中国");
    let h = &s1[0..1]; // Modify this line to fix the error, tips: `h` only takes 1 byte in UTF8 format
    assert_eq!(h, "h");
    // => You can't use index to access a char in a string, but you can use slice &s1[start..end].

    let h1 = &s1[3..5]; // Modify this line to fix the error, tips: `中`  takes 3 bytes in UTF8 format
    assert_eq!(h1, "中");

    println!("Success!");

    // 12
    for c in "你好，世界".chars() {
        println!("{}", c)
    }

    // 13
    let s = "The 🚀 goes to the 🌑!";
    let rocket = utf8_slice::slice(s, 4, 5);
}

fn greeting(s: String) {
    println!("{}", s)
}

fn greetings(s: &str) {
    println!("{}", s)
}

// String is a heap-allocated string type that owns its contenst and is mutable
// &str is an immutable(UTF-8) bytes in memory, it does not owns nad immutable
