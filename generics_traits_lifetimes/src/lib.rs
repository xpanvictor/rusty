pub trait Summary {
    fn attach_author(&self) -> String;
    fn summarize(&self) -> String {
        String::from("Read more...")
    }
}

pub fn notify(item: &impl Summary) {
    println!("{}", item.summarize());
}