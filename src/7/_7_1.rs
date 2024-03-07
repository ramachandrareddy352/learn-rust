// Fill in the blanks
fn main() {
    // 1
    let n = 5;

    if n < 0 {
        println!("{} is negative", n);
    } else if n > 0 {
        println!("{} is positive", n);
    } else {
        println!("{} is zero", n);
    }

    // 2
    let n: i32 = 25;

    let big_n: i32 = if n < 10 && n > -10 {
        println!(", and is a small number, increase ten-fold");
        10 * n
    } else {
        println!(", and is a big number, halve the number");
        n / 2.0 as i32
    };

    println!("{} -> {:?}", n, big_n);

    // 3
    for n in 1..100 {
        if n == 100 {
            panic!("NEVER LET THIS RUN")
        }
    }

    // 4
    let names: [String; 2] = [String::from("liming"), String::from("hanmeimei")];
    for name in names.clone() {
        println!("{}", name);
    }
    println!("{:?}", names); // we can use two methods

    for name in &names {
        println!("{}", name);
    }
    println!("{:?}", names);

    let numbers: [i32; 3] = [1, 2, 3];
    // The elements in numbers are Copy，so there is no move here
    for n in numbers {
        println!("{}", n);
    }
    println!("{:?}", numbers);

    // 5
    let a: [i32; 4] = [4, 3, 2, 1];
    println!("{:?}", a.iter().enumerate()); // creates a iter list to traversal through it
                                            // Iterate the indexing and value in 'a'
    for (i, v) in a.iter().enumerate() {
        println!("The {}th element is {}", i + 1, v);
    }

    // 6
    // A counter variable
    let mut n = 1;
    // Loop while the condition is true
    while n < 10 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
        n += 1;
    }
    println!("n reached {}, so loop is over", n);

    // 7
    let mut n = 0;
    for i in 0..=100 {
        if n == 66 {
            break;
        }
        n += 1;
    }
    assert_eq!(n, 66);

    // 8
    let mut n = 0;
    for i in 0..=100 {
        if n != 66 {
            n += 1;
            continue; // after continue it terminates to starting of loop
        }
        break;
    }
    assert_eq!(n, 66);

    // 9
    let mut count = 0u32;
    println!("Let's count until infinity!");

    // Infinite loop
    loop {
        count += 1;
        if count == 3 {
            println!("three");
            // Skip the rest of this iteration
            continue;
        }
        println!("{}", count);
        if count == 5 {
            println!("OK, that's enough");
            break;
        }
    }
    assert_eq!(count, 5);

    // 10
    let mut counter = 0;
    let result: i32 = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // result is stored
        }
    };
    assert_eq!(result, 20);

    // 11
    let mut count = 0;
    // in loops we can use lables('outer, 'inner1, 'inner2) to continue or break to do actions on  particular loops
    'outer: loop {
        'inner1: loop {
            if count >= 20 {
                // This would break only the inner1 loop
                break 'inner1; // `break` is also works.
            }
            count += 2;
        } // after completing count - 20

        count += 5; // count - 25

        'inner2: loop {
            if count >= 30 {
                // This breaks the outer loop
                break 'outer;
            }
            // This will continue the outer loop
            continue 'outer;
        }
    }
    assert!(count == 30);
}
