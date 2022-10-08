
enum IpAddr {
    V4 (u8, u8, u8, u8),
    V6 (String)
}

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32)
}

impl Message {
    fn call(&self) {
        
    }
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);

    let loopbac = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("Hello world!"));
    m.call();

    let absent_number: Option<i32> = None;

    let x: i8 = 8;
    let y: Option<i8> = Some(5);
    println!("The sum of both isn't possible: {}", {x+y})
}
