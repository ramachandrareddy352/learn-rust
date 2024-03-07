fn main() {
    // example
    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // This expression will be assigned to `y`
        x_cube + x_squared + x // return value is not have semicolon
    };

    let z = {
        // The semicolon suppresses this expression and `()` is assigned to `z`
        2 * x; // here result is empty(), because semicolon is used
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);

    println!("x is {}", x);
    println!("y is {}", y);
    // println!("z is {}", z);

    // 1
    let v = {
        let mut x: i32 = 1;
        x += 2; // after initialize values are not assaigned to expressions
        x
    };
    assert_eq!(v, 3);
    println!("Success!");

    let v = {
        let mut x = 1;
        x += 2
    };
    assert_eq!(v, ());
    println!("Success!");

    // 2
    let v = {
        let x = 3;
        x
    };
    let v = { 3 }; // to return 3 both are same outputs
    assert!(v == 3);
    println!("Success!");

    // 3
    let s = sum(1, 2);
    assert_eq!(s, 3);
    println!("Success!");

    assert_eq!((), {}); // it is true
}

// to return any i32 value don't use semicolons to return
fn sum(x: i32, y: i32) -> i32 {
    x + y // for returning a statement we do not have to use semicolon
}
