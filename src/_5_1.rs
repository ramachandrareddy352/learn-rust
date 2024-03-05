fn main() {
    // Use as many approaches as you can to make it work
    let x = String::from("Hello world"); // here x stores the poinetr of string in heap memory
    println!("{}", x);
    // strinng stores at stack with pointer of heap
    // value of string is stored in heap
    let y = x;
    let z = y.clone(); // for a same vaules stored in memory have two memory locations with two pointers
                       // println!("{}", x); // heer x is no longer available because it is dropped
    println!("{}", y); // after assaigning x to y then x is removed in memory
    println!("{}", z);

    // 2
    let s1 = String::from("Hello world");
    let s2 = take_ownership(s1);
    // here th eownership is changed to s2 and we cannnot no lonegr use s1 because it is dropped
    println!("{}", s2);

    // 3
    let s = give_ownership();
    println!("{}", s);

    // 4
    let s = String::from("Hello World");
    print_str(s.clone()); // here ownership is not transfering we just sending copy of data
    println!("{}", s);

    // 5
    let x: (i32, i32, (), &str) = (1, 2, (), "hello");
    let y: (i32, i32, (), &str) = x.clone();
    println!("{:?}, {:?}", x, y);

    // 6
    let s = String::from("Hello ");
    println!("{}", s);
    let mut s1 = s;
    s1.push_str("World!"); // append the data in heap at same pointer address
    println!("{}", s1);
    println!("Success!");

    // 7
    let x = Box::new(5);
    let mut y = Box::new(1); // update this line, don't change other lines!
    *y = 10;
    println!("{}", y); // pointer
    println!("{}", *y); // value of pointer
    assert_eq!(*x, 5);
    println!("Success!");

    // partial example
    #[derive(Debug)]
    struct Person {
        name: String,
        age: Box<u8>,
    }

    let person: Person = Person {
        name: String::from("Alice"),
        age: Box::new(20),
    };

    // `name` is moved out of person, but `age` is referenced
    let Person { name, ref age } = person;

    println!("The person's age is {}", age);
    println!("The person's name is {}", name);

    // Error! borrow of partially moved value: `person` partial move occurs
    //println!("The person struct is {:?}", person);
    // `person` cannot be used but `person.age` can be used as it is not moved
    println!("The person's age from person struct is {}", person.age);

    // 8
    let t = (String::from("hello"), String::from("world"));
    println!("{:?}", t); // print entire tuple
    let _s = t.0; // first element of tuple ownership is changed
                  // Modify this line only, don't use `_s`
    println!("{:?}", t.1);

    // 9
    let t = (String::from("hello"), String::from("world"));
    let (s1, s2) = (t.0.clone(), t.1.clone());
    // let (s1,s2) = t.clone()
    println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
}

fn print_str(s: String) {
    println!("{}", s)
}

// Only modify the code below!
fn give_ownership() -> String {
    let s = String::from("Hello world");
    // Convert String to Vec
    let _s = s.as_bytes(); // here we are not coping we are geting values into _s of bytes
    println!("{:?}", _s);
    println!("{:?}", s);
    s
}

// Only modify the code below!
fn take_ownership(s: String) -> String {
    println!("{}", s);
    s
}
// for a single memory pointer their is only one variable(owner) to assaign.
