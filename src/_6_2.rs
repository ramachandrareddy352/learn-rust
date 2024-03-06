fn main() {
    // 1
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    assert!(arr.len() == 5);

    // 2
    // We can ignore parts of the array type or even the whole type, let the compiler infer it for us
    let arr0: [i32; 3] = [1, 2, 3];
    let arr: [char; 3] = ['a', 'b', 'c'];

    // Arrays are stack allocated, `std::mem::size_of_val` returns the bytes which an array occupies
    // A char takes 4 bytes in Rust: Unicode char
    assert!(std::mem::size_of_val(&arr) == 4 * 3);

    // 3
    let list: [i32; 100] = [1; 100]; // [1,1,1,1,....,1,1,] => 100 1's are present
    assert!(list[0] == 1);
    assert!(list.len() == 100);

    // 4
    let _arr: [i32; 3] = [1, 2, 3];

    // 5
    let arr = ['a', 'b', 'c'];
    let ele = arr[0];
    assert!(ele == 'a');

    // 6
    let names: [String; 2] = [String::from("Sunfei"), "Sunface".to_string()];
    let name0: &String = names.get(0).unwrap();
    let _name1: &String = &names[1]; // max len = 1
}
