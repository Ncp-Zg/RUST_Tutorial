#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn width(&self) -> bool {
        self.width > 0
    }
    fn can_hold(&self, other: &Rectangle) ->bool {
        self.width > other.width && self.height > other.height
    }
}


    


fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width:dbg!(30 * scale),
        height:50,
    };
    let rect2 = Rectangle {
        width:10,
        height:50,
    };
    let rect3 = Rectangle {
        width:10,
        height:10,
    };

    println!(
        "The area is the rectangle is {:#?} square pixels.", rect1.width()
    );
    println!(
        "Can rect1 hold rect2? {}", rect1.can_hold(&rect2)
    );
    println!(
        "Can rect1 hold rect3? {}", rect1.can_hold(&rect3)
    );
}

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }
