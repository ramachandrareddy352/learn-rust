// Fix the error
struct Person {
    name: String,
    age: u8,
    hobby: String,
}

struct Unit;
trait SomeTrait {
    // ...Some behaviors defined here.
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// We don't care about what fields  are  in the Unit, but we care about its behaviors.
// So we use a struct with no fields and implement some behaviors for it
impl SomeTrait for Unit {}

struct Person1 {
    name: String,
    age: u8,
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
fn main() {
    // 1
    let age: u8 = 30;
    let mut p: Person = Person {
        name: String::from("sunface"),
        age,
        hobby: String::from("sleeping"),
    };
    p.hobby = "running".to_string();
    println!("{:?}", p.hobby);

    let p1: Person = Person {
        name: String::from("sunface"),
        ..p // instantiating remaining filedswith values of `p` to `p1`
    };
    println!("{:?}", p1.hobby);

    // 2
    let u: Unit = Unit;
    do_something_with_unit(u);
    println!("Success!");

    // 3
    let v = Color(0, 127, 255);
    check_color(v);

    // 4
    let age = 18;
    let mut p = Person1 {
        name: String::from("sunface"),
        age,
    };

    p.age = 30;
    p.name = String::from("sunfei");
    println!("Success!");

    // 6
    let u1 = User {
        email: String::from("someone@example.com"),
        username: String::from("sunface"),
        active: true,
        sign_in_count: 1,
    };
    let u2 = set_email(u1);
    println!("Success!");

    // 7
    let scale = 2;
    let rect1: Rectangle = Rectangle {
        width: dbg!(30 * scale), // Print debug info to stderr and assign the value of  `30 * scale` to `width`
        height: 50,
    };
    dbg!(&rect1); // Print debug info to stderr
    println!("{:?}", rect1); // Print debug info to stdout

    // parial move example
    #[derive(Debug)]
    struct Person2 {
        name: String,
        age: Box<u8>,
    }

    let person2 = Person2 {
        name: String::from("Alice"),
        age: Box::new(20),
    };
    // `name` is moved out of person, but `age` is referenced
    let Person2 { name, ref age } = person2;
    println!("The person's age is {}", age);
    println!("The person's name is {}", name);

    // Error! borrow of partially moved value: `person` partial move occurs
    //println!("The person struct is {:?}", person);
    // `person` cannot be used but `person.age` can be used as it is not moved
    println!("The person's age from person struct is {}", person2.age);

    // 8
    #[derive(Debug)]
    struct File {
        name: String,
        data: String,
    }

    let f = File {
        name: String::from("readme.md"),
        data: "Rust By Practice".to_string(),
    };
    let _name = f.name.clone();

    println!("{}, {}, {:?} ", _name, f.data, f); // accessing entire struct is not possible
}

fn set_email(u: User) -> User {
    User {
        email: String::from("contact@im.dev"),
        ..u
    }
}

fn check_color(p: Color) {
    let Color(x, y, z) = p;
    assert_eq!(x, 0);
    assert_eq!(p.1, 127);
    assert_eq!(z, 255);
}

fn do_something_with_unit(u: Unit) {}
