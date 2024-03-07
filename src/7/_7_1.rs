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
    let n = 5;

    let big_n = 20;
    if n < 10 && n > -10 {
        println!(", and is a small number, increase ten-fold");
        10 * n
    } else {
        println!(", and is a big number, halve the number");
        n / 2.0 as i32
    };

    println!("{} -> {}", n, big_n);

    // 3
    for n in 1..100 {
        if n == 100 {
            panic!("NEVER LET THIS RUN")
        }
    }

    // 4
    let names: [String; 2] = [String::from("liming"), String::from("hanmeimei")];
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
    // Iterate the indexing and value in 'a'
    for (i, v) in a.iter().enumerate() {
        println!("The {}th element is {}", i + 1, v);
    }
}
