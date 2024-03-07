enum Number {
    Zero,
    One,
    Two,
}

enum Number1 {
    // not allowed of floating values
    Zero = 5,
    One, // 6
    Two, // 7
}

// C-like enum
enum Number2 {
    // changes explictily
    Zero = 0,
    One = 6,
    Two = 2,
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    // 1
    // a enum variant can be converted to a integer by `as`
    assert_eq!(Number::One as u8 + 5, Number1::One as u8);
    assert_eq!(Number1::One as u8, Number2::One as u8);

    // 2
    let msg1 = Message::Move { x: 1, y: 2 }; // instantiating with x = 1, y = 2
    let msg2 = Message::Write(String::from("hello, world")); // instantiating with "hello, world!"
    let msg3 = Message::ChangeColor(1, 2, 3);

    // 3
    let msg = Message::Move { x: 1, y: 1 };
    if let Message::Move { x: a, y: b } = msg {
        assert_eq!(a, b);
    } else {
        panic!("NEVER LET THIS RUN！");
    }

    // 4
    let msgs: [Message; 3] = [
        Message::Quit,
        Message::Move { x: 1, y: 3 },
        Message::ChangeColor(255, 255, 0),
    ];
    for msg in msgs {
        show_message(msg)
    }

    // 6
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    if let Some(n) = six {
        println!("{}", n);
        println!("Success!");
    }
    panic!("NEVER LET THIS RUN！");
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn show_message(msg: Message) {
    println!("{:?}", msg); // we ahve to use debug trait to print custome results
}
