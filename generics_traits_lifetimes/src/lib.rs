use std::fmt::Display;

pub trait Summary {
    fn attach_author(&self) -> String;
    fn summarize(&self) -> String {
        String::from("Read more...")
    }
}

pub fn notify(item: &impl Summary) {
    println!("{}", item.summarize());
}

pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    }else {
        y
    }
}

pub fn longest_with_an_announcement<'a, T> (
    x: &'a str,
    y: &'a str,
    ann: T
) -> &'a str
    where T: Display,
{
    println!("Announcement: {}", ann);
    longest(x, y)
}