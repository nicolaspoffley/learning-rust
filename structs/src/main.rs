struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: i32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle::square(50);
    
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2 ? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3 ? {}", rect1.can_hold(&rect3));
}
