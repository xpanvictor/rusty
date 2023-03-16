use std::io::{self, Read};
use std::fs::{self, File};
use std::path::Path;

fn read_username_from_file() -> Result<String, io::Error> {
    // ----------Normal implementation-----------
    // let file = File::open("src/username.txt");
    //
    // let mut file = match file {
    //     Ok(file) => file,
    //     Err(e) => return Err(e)
    // };
    //
    // let mut s = String::new();
    //
    // match file.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e)
    // }

    // -----------Using the ? operator implementation--------
    // let file = File::open("/src/username.txt")?;
    // let mut s = String::new();
    // file.read_to_string(&mut s)?;
    // Ok(s)

    // ----------Using ? with chaining-------------------
    // let mut s = String::new();
    //
    // File::open("username.txt")?.read_to_string(&mut s)?;
    //
    // Ok(s)

    // ----------Using std function
    fs::read_to_string(Path::new("./src/username.txt"))
}

fn main() {
    println!("Hello, world!");

    let username_result = read_username_from_file();
    let otp: u32 = 32849;
    let mut merged_data = String::new();

    let username = username_result.unwrap_or_else(|error| {
        panic!("Failed at retrieving username, {:?}", error)
    });

    merged_data = format!("{} {}", username, otp);
    println!("Your merged data is {merged_data}")
}
