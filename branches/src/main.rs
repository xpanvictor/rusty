fn main() {
    // // conditional
    // let number = if 2 > 7 {7} else {2};

    // if number < 5 {
    //     println!("Condition was true")
    // } else {
    //     print!("Condition was false")
    // }

    // // repetitions with loop -> loop, while, for
    // loop
    // loop {
    //     println!("Will continue forever unless a break is in the block")
    // }
    // returning data from loop
    // let mut counter = 0;
    
    // let result = loop {
    //     counter += 1;

    //     if counter == 10 {
    //         break counter * 2;
    //     }
    // };

    // println!("The final count is {result}")

    // labeled loop
    // 'counting_up: loop {
    //     println!("count = {counter}");
    //     let mut remaining = 10;

    //     loop {
    //         println!("remaining = {remaining}");
    //         if remaining == 9 {
    //             break;
    //         }
    //         if counter == 2 {
    //             break 'counting_up;
    //         }
    //         remaining -= 1;
    //     }

    //     counter += 1;
    // }

    // println!("End count = {counter}")

    // // while loop
    // let mut number = 3;

    // while number != 0 {
    //     println!("{number}!");
    //     number -= 1;
    // }
    // println!("Liftoff")

    // // for loop
    // let a = [3; 5];
    // for element in a {
    //     println!("Value is {element}!")
    // }

    // countdown with for loop
    for number in (1..4).rev() {
        println!("In {number}!");
    }
    println!("Liftoff xpan!!!")

}
