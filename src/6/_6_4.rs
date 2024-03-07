fn main() {
    // 1
    let _t0: (u8, i16) = (0, -1);
    let _t1: (u8, (i16, u32)) = (0, (-1, 1));
    let t: (u8, u16, i64, &str, String) = (1u8, 2u16, 3i64, "hello", String::from(", world"));
    println!("{:?}", t);

    // 2
    let t = ("i", "am", "sunface");
    assert_eq!(t.1, "am");

    // 3
    // long tuples cannot be printed, but we can use the tuple
    let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    println!("too long tuple: {:?}", too_long_tuple);

    // 4
    let tup = (1, 6.4, "hello");
    let (x, z, y) = tup;
    assert_eq!(x, 1);
    assert_eq!(y, "hello");
    assert_eq!(z, 6.4);

    // 5
    let (mut x, mut y, mut z);
    (y, z, x) = (1, 2, 3);
    (y, z, x) = (1, 2, 3);
    assert_eq!(x, 3);
    assert_eq!(y, 1);
    assert_eq!(z, 2);

    // 6
    let nums: (i32, i32) = (2, 3);
    let (x, y) = sum_multiply(nums);
    assert_eq!(x, 5);
    assert_eq!(y, 6);
}

fn sum_multiply(nums: (i32, i32)) -> (i32, i32) {
    (nums.0 + nums.1, nums.0 * nums.1)
}
