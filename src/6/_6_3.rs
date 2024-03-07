fn main() {
    // 1
    let arr: [i32; 3] = [1, 2, 3];
    let s1: &[i32] = &arr[0..2]; // => [1,2]
    let s2: &str = "hello, world";
    println!("{:?}", s1);
    println!("{:?}", s2);

    // 2
    let arr: [char; 3] = ['中', '国', '人'];
    let slice: &[char] = &arr[..2]; // ['中', '国']
                                    // Modify '8' to make it work
                                    // TIPS: slice( reference ) IS NOT an array, if it is an array, then `assert!` will be passed: Each of the two chars '中' and '国'  occupies 4 bytes, 2 * 4 = 8
    assert!(std::mem::size_of_val(&slice) == 16);
    // here slice holds the pointer values, a pointer holds 1 byte = 8 bits of size and there are 2 pointers are holding in slice

    // 3
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let slice: &[i32] = &arr[1..4];
    assert_eq!(slice, &[2, 3, 4]);
    println!("{:?}", slice);

    // 4
    let s = String::from("hello");
    let slice1 = &s[0..2];
    let slice2 = &s[..2];
    assert_eq!(slice1, slice2);

    // 5
    let s = "你好，世界";
    let slice = &s[0..3];
    assert!(slice == "你");

    // 6
    let mut s = String::from("hello world");

    // Here, &s is `&String` type, but `first_letter` needs a `&str` type.
    // It works because `&String` can be implicitly converted to `&str. If you want to know more, this is called `Deref coercion`.
    let letter = first_letter(&s);
    println!("the first letter is: {}", letter);
}

fn first_letter(s: &str) -> &str {
    &s[..1]
}
