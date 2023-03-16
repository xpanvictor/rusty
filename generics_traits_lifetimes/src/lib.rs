pub trait Summary {
    fn attach_author(&self) -> String;
    fn summarize(&self) -> String {
        String::from("Read more...")
    }
}
