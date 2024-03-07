struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

// 2
#[derive(Debug)]
struct TrafficLight {
    color: String,
}

// we can use multiple implimentation blocks
impl TrafficLight {
    pub fn show_state(&self) {
        println!("the current state is {}", self.color);
    }
}
impl TrafficLight {
    pub fn change_state(&mut self) {
        self.color = "green".to_string()
    }
}

fn main() {
    // 1
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    assert_eq!(rect1.area(), 1500);
    println!("Success!");

    // 2
    let mut light = TrafficLight {
        color: "red".to_owned(),
    };
    // Don't take the ownership of `light` here
    light.show_state();
    light.change_state();
    light.show_state();
    // ..otherwise, there will be an error below
    println!("{:?}", light);
}
