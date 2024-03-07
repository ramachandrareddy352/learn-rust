enum Direction {
    East,
    West,
    North,
    South,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum MyEnum {
    Foo,
    Bar,
}

enum Foo {
    Bar(u8),
}

enum Foo2 {
    Bar,
    Baz,
    Qux(u32),
}

fn main() {
    // 1
    let dire = Direction::West;
    match dire {
        Direction::East => println!("East"),
        Direction::North | Direction::South => {
            // Matching South or North here
            println!("South or North");
        }
        _ => println!("West"), // remaining cases
    };

    // 2
    let boolean = true;
    // boolean = true => binary = 1
    // boolean = false =>  binary = 0
    let binary = match boolean {
        true => 1,
        false => 0,
    };
    assert_eq!(binary, 1);

    // 3
    let msgs: [Message; 3] = [
        Message::Quit,
        Message::Move { x: 1, y: 3 },
        Message::ChangeColor(255, 255, 0),
    ];
    for msg in msgs {
        show_message(msg)
    }

    // 4
    let alphabets: [char; 7] = ['a', 'E', 'Z', '0', 'x', '9', 'Y'];
    for ab in alphabets {
        assert!(matches!(ab, 'a'..='z' | 'A'..='Z' | '0'..='9'))
    }

    // 5
    let mut count: i32 = 0;
    let v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];
    for e in v {
        if matches!(e, MyEnum::Foo) {
            count += 1;
        }
    }
    assert_eq!(count, 2);

    // 6
    let o = Some(17);
    // Remove the whole `match` block, using `if let` instead
    match o {
        Some(i) => {
            println!("This is a really long string and `{:?}`", i);
            println!("Success!");
        }
        _ => {}
    };

    if let Some(i) = o {
        println!("This is a really long string and `{:?}`", i);
        println!("Success!");
    }

    // 7
    let a = Foo::Bar(1);
    if let Foo::Bar(i) = a {
        println!("foobar holds the value: {}", i);
        println!("Success!");
    }

    // 8
    let a = Foo2::Qux(10);
    // Remove the codes below, using `match` instead
    if let Foo2::Bar = a {
        println!("match foo::bar")
    } else if let Foo2::Qux(10) = a {
        println!("match foo::Qux")
    } else {
        println!("match others")
    }

    match a {
        Foo2::Bar => println!("match foo::bar"),
        Foo2::Qux(10) => println!("match foo::Qux"),
        _ => println!("match others"),
    }

    // 9
    let age = Some(30);
    if let Some(age) = age {
        // create a new variable with the same name as previous `age`
        assert_eq!(age, 30);
    } // the new variable `age` goes out of scope here

    match age {
        // match can also introduce a new shadowed variable
        Some(age) => println!("age is a new variable, it's value is {}", age),
        _ => (),
    }
}

fn show_message(msg: Message) {
    match msg {
        Message::Move { x: a, y: b } => {
            // match  Message::Move
            assert_eq!(a, 1);
            assert_eq!(b, 3);
            println!("Move");
        }
        Message::ChangeColor(r, g, b) => {
            assert_eq!(g, 255);
            assert_eq!(b, 0);
            println!("Change color");
        }
        _ => println!("no data in these variants"),
    }
}
