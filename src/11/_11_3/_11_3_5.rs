// FIX the errors with least changes
// DON'T remove any code line
use std::collections::HashMap;
fn main() {
    // example
    let mut map: HashMap<i32, i32> = HashMap::with_capacity(100);
    map.insert(1, 2);
    map.insert(3, 4);
    // Indeed ,the capacity of HashMap is not 100, so we can't compare the equality here.
    // capacity is more because rust compiler use more to avoid hash collisions
    assert!(map.capacity() >= 100);

    // Shrinks the capacity of the map with a lower limit. It will drop
    // down no lower than the supplied limit while maintaining the internal rules
    // and possibly leaving some space in accordance with the resize policy.

    map.shrink_to(50);
    assert!(map.capacity() >= 50);

    // Shrinks the capacity of the map as much as possible. It will drop
    // down as much as possible while maintaining the internal rules
    // and possibly leaving some space in accordance with the resize policy.
    map.shrink_to_fit();
    assert!(map.capacity() >= 2);
    println!("Success!");

    // 1
    let v1 = 10;
    let mut m1 = HashMap::new();
    m1.insert(v1, v1);
    println!("v1 is still usable after inserting to hashmap : {}", v1);

    let v2 = "hello".to_string(); // or: let v2: &str = "hello";
    let mut m2 = HashMap::new();
    // Ownership moved here
    m2.insert(v2.as_str(), v1);

    assert_eq!(v2, "hello");

    println!("Success!");
}
