fn main() {
    // 1
    let x = 5;
    let p = &x;
    println!("the memory address of x is {:p}", p); // :p is pointer variable

    // 2
    let x = 5;
    let y = &x;
    assert_eq!(5, *y);
    assert_eq!(5, x);

    // 3
    let mut s = String::from("hello, ");
    borrow_object(&s); // here we have to pass reference not the string

    // 4
    let mut s = String::from("hello, ");
    println!("{}", s);
    push_str(&mut s);
    println!("{}", s);

    // 5
    let mut s = String::from("hello, ");
    let p = &mut s;
    p.push_str("world"); // we have updated `s` uisng `p` reference
    println!("{}", s);

    // 6
    let c = '中';
    let r1 = &c;
    let ref r2 = c; // either use both methods
                    // let r2 = &c;
    assert_eq!(*r1, *r2);
    // Check the equality of the two address strings
    assert_eq!(get_addr(r1), get_addr(r2));
    println!("Success!");

    // 7
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s; // while reading we dont need mut reference
    println!("{}, {}", r1, r2);

    // 8
    let mut s = String::from("hello, ");
    borrow__object(&mut s);
    println!("Success!");

    // 9
    let mut s = String::from("hello, ");
    borrow_object(&s);
    s.push_str("world");
    println!("Success!");

    // 10
    let mut s = String::from("hello, ");
    let r1 = &mut s;
    r1.push_str("world");
    let r2 = &mut s;
    r2.push_str("!");
    // println!("{}", r1);  // after pointing `s` to `r2` we cannot use `r1` pointer
    println!("{}", r2);
}

fn borrow__object(s: &mut String) {}

// Get memory address string
fn get_addr(r: &char) -> String {
    format!("{:p}", r)
}

fn push_str(s: &mut String) {
    s.push_str("world")
}

fn borrow_object(s: &String) {}
