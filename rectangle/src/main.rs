
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
        self.height * self.width
    }
}
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50
    };
    // saying rust print the rect1 struct using debug format, which needs an outer attribute
    println!("The rectangle is {:#?}", rect1);
    println!("The area of the rectangle is {} square pixels.", rect1.area());
}
