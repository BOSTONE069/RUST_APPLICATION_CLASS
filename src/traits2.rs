pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.content)
}
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet{
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

/// This code defines a trait named `Summary` with a method `summarize` that returns a `String`. The
/// implementation of this trait provides a default behavior for the `summarize` method, which returns
/// the string "Read more..." if not overridden by types implementing the `Summary` trait. This allows
/// types like `NewsArticle` and `Tweet` to implement the `Summary` trait and provide their own custom
/// implementations for the `summarize` method if needed.
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("Read more...")
    }
}

pub fn social_traits(){
    let tweet = Tweet{
        username: String::from("@bostone1234"),
        content: String::from("Hello World!"),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle{
        author: String::from("Bostone Ochieng"),
        headline: String::from("The sky is falling"),
        content: String::from("The sky is not actually falling")
    };

    println!("Tweet Summary: {}", tweet.summarize());
    println!("Article summary: {}", article.summarize());
}