use generics_traits_lifetimes::{longest_with_an_announcement, notify, Summary};

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String
}

impl Summary for NewsArticle {
    fn attach_author(&self) -> String {
        format!("@{}", self.author)
    }
}

// read can be num or false for not read
struct Tweet<T > {
    author: String,
    content: String,
    reply: bool,
    read: T
}

impl Summary for Tweet<u32> {
    fn attach_author(&self) -> String {
        format!("@{}", self.author)
    }
    fn summarize(&self) -> String {
        format!("Tweet read {} times; by {}: {}...", self.read, self.attach_author(), self.content)
    }
}

fn main() {
    let news_article = NewsArticle {
        headline: String::from("How to catch a tiger"),
        location: String::from("Nigeria"),
        author: String::from("xpanvictor"),
        content: String::from(
            "Well, forest, tranquilizers, shoot, chain and simple, you have a tiger. Disclaimer, you could die"
        )
    };
    println!("Summary of news article: {}", news_article.summarize());
    let tweet = Tweet {
        author: String::from("xpan"),
        reply: false,
        content: String::from("Hello world, I love dancing, lol, lfg!"),
        read: 13
    };
    notify(&tweet);

    let longest_author = longest_with_an_announcement(&news_article.author, &tweet.author, &news_article.location);
    println!("The longest author is {}", longest_author);
}