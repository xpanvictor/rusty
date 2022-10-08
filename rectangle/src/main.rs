
// variables
// fn main() {
//     let width1 = 30;
//     let height1 = 50;

//     println!("The area of the rectangle is {} square pixels.", area(width1, height1));
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }


// tuples
// fn main() {

//     let rect1 = (30, 50);

//     println!("The area of the rectangle is {} square pixels.", area(rect1));
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// structs
// outer attribute to allow us print this struct with debug format
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50
    };

    let rect2 = Rectangle {
        width: 20,
        height: 20
    };

    let square1 = Rectangle::square(64);
    // saying rust print the rect1 struct using debug format, which needs an outer attribute
    println!("The rectangle is {:#?}", rect1);
    println!("The area of the rectangle is {} square pixels.", rect1.area());

    println!("Rectangle 1 can {} hold rectangle 2", if rect1.can_hold(&rect2) {"definitely"} else {"not"});
    println!("Rectangle 1 can {} hold square 1", if rect1.can_hold(&square1) {"definitely"} else {"not"});
}
