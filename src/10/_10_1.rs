use std::process::Output;

// Fill in the blanks to make it work
struct A; // Concrete type `A`.
struct S(A); // Concrete type `S`.
struct SGen<T>(T); // Generic type `SGen`.

fn reg_fn(_s: S) {}
fn gen_spec_t(_s: SGen<A>) {}
fn gen_spec_i32(_s: SGen<i32>) {}
fn generic<T>(_s: SGen<T>) {}

fn main() {
    // 1
    // Using the non-generic functions
    reg_fn(S(A)); // Concrete type.
    gen_spec_t(SGen(A)); // Implicitly specified type parameter `A`.
    gen_spec_i32(SGen(7)); // Implicitly specified type parameter `i32`.

    // Explicitly specified type parameter `char` to `generic()`.
    generic::<char>(SGen('a'));
    generic::<f64>(SGen(7.7));
    // Implicitly specified type parameter `char` to `generic()`.
    generic(SGen('a'));
    generic(SGen(7.7)); // float to generic

    println!("Success!");

    // 2
    assert_eq!(5, sum(2i8, 3i8));
    assert_eq!(50, sum(20, 30));
    assert_eq!(2.46, sum(1.23, 1.23));

    // 3
    let integer: Point<i32> = Point { x: 5, y: 10 };
    let float: Point<f64> = Point { x: 1.0, y: 4.0 };

    // 4
    let p = Point1 {
        x: 5,
        y: "hello".to_string(),
    };

    // 5
    let x = Val { val: 3.0 };
    let y = Val {
        val: "hello".to_string(),
    };
    println!("{:?}, {:?}", x.value(), y.value());

    // 6
    let p1 = Point2 { x: 5, y: 10 };
    let p2 = Point2 {
        x: "Hello",
        y: '中',
    };
    let p3 = p1.mixup(p2);

    assert_eq!(p3.x, 5);
    assert_eq!(p3.y, '中');

    // 7
    let p: Point3<f32> = Point3 {
        x: 5_f32,
        y: 10_f32,
    };
    println!("{:?}", p.distance_from_origin());
}

struct Point3<T> {
    x: T,
    y: T,
}

impl Point3<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct Point2<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point2<T, U> {
    fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

struct Val<T> {
    val: T,
}

impl<T> Val<T> {
    fn value(&self) -> &T {
        &self.val
    }
}

struct Point1<T, R> {
    x: T,
    y: R,
}

struct Point<T> {
    x: T,
    y: T,
}

fn sum<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}
