use rand::Rng;
use std::cmp::Ordering;
use std::io;

// to use a function to generate the random number
fn gen_secret_number() -> u32 {
    rand::thread_rng()
        // generate random num between range start..=end
        .gen_range(1..=100)
}

fn main() {
    // instantiating the random number gen
    // thread_rng() instantiates rng n seeds it
    // let secret_number = rand::thread_rng()
    //     // generate random num between range start..=end
    //     .gen_range(1..=100);
    let secret_number = gen_secret_number();

    println!("Guess the number!");
    // the loop system
    loop {
        // introducing the game
        println!("Please input the number: {}", 34);

        // creating a mutable variable with `mut`; without which it'll be immutable
        let mut guess = String::new();

        // stdin() instantiates the std library for io operations
        io::stdin()
            // this takes input from terminal and pipes it to the mutable reference of the guess var
            .read_line(&mut guess)
            // handle error also returns either Ok with value of read or Err with trace
            .expect("Failed to read line!");

        // convert guess string to unsigned 32b integer
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        // match => switch and Ordering => case<enum> ref js
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                // break the loop
                break;
            }
        }
    }
}
