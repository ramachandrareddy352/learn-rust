use std::mem;
fn main() {
    // 1
    let mut s: String = String::from("hello, ");
    s.push_str("world");
    s.push('!');
    move_ownership(s.clone());
    assert_eq!(s, "hello, world!");

    // 2
    let s = String::from("hello, world");
    let slice1: &str = "hello, world"; // In two ways
    assert_eq!(slice1, "hello, world");

    let slice2 = "hello";
    assert_eq!(slice2, "hello");

    let mut slice3: String = String::from("hello, world");
    slice3.push('!');
    assert_eq!(slice3, "hello, world!");

    // 3
    let s: String = String::from("hello, world!");
    let slice: &str = &s;
    let s: String = slice.to_string();
    assert_eq!(s, "hello, world!");

    // 4
    let s = String::from("hello, 世界");
    let slice1 = &s[0..1]; //tips: `h` only takes 1 byte in UTF8 format
    assert_eq!(slice1, "h");
    let slice2 = &s[7..10]; // Tips: `中`  takes 3 bytes in UTF8 format
    assert_eq!(slice2, "世");
    // Iterate through all chars in s
    for (i, c) in s.chars().enumerate() {
        // chars().enumerate() gives [index, value at that index]
        if i == 7 {
            assert_eq!(c, '世')
        }
    }

    // 5
    let mut s = String::new();
    s.push_str("hello");
    // Some bytes, in a vector
    let v: Vec<u8> = vec![104, 101, 108, 108, 111];
    // Turn a byte's vector into a String
    let s1: String = String::from_utf8(v).unwrap();
    assert_eq!(s, s1);

    // 6
    let mut s = String::new();
    println!("{}", s.capacity());
    for _ in 0..2 {
        s.push_str("hello");
        println!("after-{}", s.capacity());
    }

    // 7
    let story = String::from("Rust By Practice");
    // Prevent automatically dropping of the String's data
    let mut story = mem::ManuallyDrop::new(story);

    let ptr = story.as_mut_ptr();
    let len = story.len();
    let capacity = story.capacity();

    println!("{:?}", ptr);
    println!("{:?}", len);
    println!("{:?}", capacity);

    assert_eq!(16, len);

    // We can rebuild a String out of ptr, len, and capacity. This is all unsafe because we are responsible for making sure the components are
    // valid:
    let s = unsafe { String::from_raw_parts(ptr, len, capacity) };
    assert_eq!(*story, s);
}

fn move_ownership(s: String) {
    println!("ownership of \"{}\" is moved here!", s)
}
