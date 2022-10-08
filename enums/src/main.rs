
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

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn value_in_cents (coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", {state});
            25
        }
    }
}

fn plus_one (x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1)
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
    // println!("The sum of both isn't possible: {}", {x+y})

    value_in_cents(Coin::Quarter(UsState::Alaska));

    // if let to handle verbose matching where cases are ignored like with match _ arm.
    let coin = Coin::Quarter(UsState::Alabama);
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state)
    } else {
        count += 1;
    }
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(absent_number);
    println!("{:?}", {six});
    println!("{:?}", {none});

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max)
    }
}
