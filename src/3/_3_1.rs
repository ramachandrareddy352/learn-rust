fn main() {
    // warnings
    let x: i32 = 5;
    let _y: i32;
    let _z: i32 = 0;

    assert_eq!(x, 5);
    assert_eq!(_z, 0);
    println!("Success!");

    // mutable
    let mut a = 1;
    a += 2;

    assert_eq!(a, 3);

    // scope of variable
    let mut x: i32 = 10; // we can re initialize the variable
    x += 20;
    {
        let y: i32 = 5;
        println!("The value of x is {} and value of y is {}", x, y);
    }
    // println!("The value of x is {} and value of y is {}", x, y);   // y is out of scope it shows error

    // function
    define_fun();

    // shadowing
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12);
    }

    assert_eq!(x, 5);

    let x = 42;
    println!("{}", x); // Prints "42".

    // shadowing and rebinding
    let mut x: i32 = 1;
    x += 7;
    // Shadowing and re-binding
    let x = x;
    // x += 3; gives error

    let _y = 4;
    // Shadowing
    let _y = "I can also be bound to text!";

    println!("Success!");

    // 8
    let (mut x, y) = (1, 2);
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Success!");

    // 9
    let (x, y);
    (x, ..) = (3, 4, 5);
    [.., y] = [1, 2, 3];
    // Fill the blank to make the code work
    assert_eq!([x, y], [3, 3]);

    println!("Success!");
}

fn define_fun() {
    let name: &str = "Rama";
    let surname = "Tavanam";

    println!("My name is {} {}", name, surname);
}

//  if we declare any variable and we are not using it then it shows warning
//  if we daclare and using some variables it shows error, if we not assaign any values to it because rust does not take any default values.
//  for not initialized and not used variable use underscore(_) to avoid warnings
// by default variables are immutable
