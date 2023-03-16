use generics_traits_lifetimes::Summary;

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String
}

impl Summary for NewsArticle {}

struct Tweet {
    author: String,
    content: String,
    reply: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("Tweet summary by {}: {}...", self.author, self.content)
    }
}

fn main() {
    let newsArticle = NewsArticle {
        headline: String::from("How to catch a tiger"),
        location: String::from("Nigeria"),
        author: String::from("xpan"),
        content: String::from(
            "Well, forest, tranquilizers, shoot, chain and simple, you have a tiger. Disclaimer, you could die"
        )
    };
    println!("Summary of news article: {}", newsArticle.summarize());
    let tweet = Tweet {
        author: String::from("xpan"),
        reply: false,
        content: String::from("Hello world, I love dancing, lol, lfg!")
    };
    println!("Summary for tweet: {}", tweet.summarize());
}