#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
struct Array<T, const N: usize> {
    data: [T; N],
}

fn check_size<T>(val: T)
where
    Assert<{ core::mem::size_of::<T>() < 768 }>: IsTrue,
{
    //...
}
fn main() {
    // 1
    let array1: [Array<i32, 3>; 3] = [
        // for cosnt generic types we have to pass its datatype and length
        Array { data: [1, 2, 3] },
        Array { data: [1, 2, 3] },
        Array { data: [1, 2, 4] },
    ];
    let array2: [Array<f32, 2>; 4] = [
        // for cosnt generic types we have to pass its datatype and length
        Array { data: [1.0, 2.0] },
        Array { data: [1.2, 3.0] },
        Array { data: [1.2, 4.3] },
        Array { data: [1.2, 4.3] },
    ];

    // 2
    let arr = [1, 2, 3];
    print_array(arr);
    let arr = ["hello", "world"];
    print_array(arr);

    // 3
    check_size([0u8; 767]);
    check_size([0i32; 191]);
    check_size(["hello你好"; 47]); // &str is a string reference, containing a pointer and string length in it, so it takes two word long, in x86-64, 1 word = 8 bytes
    check_size([(); 31].map(|_| "hello你好".to_string())); // String is a smart pointer struct, it has three fields: pointer, length and capacity, each takes 8 bytes
    check_size(['中'; 191]); // A char takes 4 bytes in Rust
    println!("Success!");
}

fn print_array<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
    println!("{:?}", arr);
}

pub enum Assert<const CHECK: bool> {}
pub trait IsTrue {}
impl IsTrue for Assert<true> {}
