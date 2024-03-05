fn main() {
    // 1
    // Don't modify the following two lines!
    let (x, y) = (1, 2);
    let s = sum(x, y);
    assert_eq!(s, 3);
    println!("Success!");

    // 2
    print();

    // 3
    // never_return();
    println!("Failed!");

    // 4
    // Diverging functions never return to the caller, so they may be used in places where a value of any type is expected.
    println!("Success!");

    // 5
    let b = true;
    let _v = match b {
        true => 1,
        // Diverging functions can also be used in match expression to replace a value of any value
        false => {
            println!("Success!");
            panic!("we have no value for `false`, but we can panic");
        }
    };
    println!("Exercise Failed if printing out this line!");
}

fn get_option(tp: u8) -> Option<i32> {
    match tp {
        1 => {
            // TODO
        }
        _ => {
            // defaut
        }
    };

    // Rather than returning a None, we use a diverging function instead
    never_return_fn()
}

// IMPLEMENT this function in THREE ways
fn never_return_fn() -> ! {
    panic!()
    // unimplemented!()
    // todo!()
}

fn never_return() -> ! {
    // Implement this function, don't modify the fn signatures
    panic!()
    // afetr executing panic it shows error
}

// Replace i32 with another type
// if we are not return anythink use -> ()
/**
 * or use
 * fn print() {
    println!("Success!");
}
 */
fn print() -> () {
    println!("Success!");
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}
// diverging functions never return values, infinite loop
