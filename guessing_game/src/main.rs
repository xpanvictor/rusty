use std::io;

fn main() {
    // introducing the game
    println!("Guess the number!");
    println!("Please input the number: ");

    // creating a mutable variable with `mut`; without which it'll be immutable
    let mut guess = String::new();

    // stdin() instantiates the std library for io operations
    io::stdin()
        // this takes input from terminal and pipes it to the mutable reference of the guess var
        .read_line(&mut guess)
        // handle error also returns either Ok with value of read or Err with trace
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
