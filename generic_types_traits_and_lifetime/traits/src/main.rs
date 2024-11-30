// We are going to implement a trait, which is common between different datatypes
// here we have built trait called summarize and implemented it's methods on both types called News
// and tweet so that we do not have to replicate the implemetnation of something again and again.
// It's like having the implemetation of class in Javascript

use std::fmt::format;

fn main() {
    let tweet = Tweet {
        username: String::from("user1"),
        content: String::from("this is my first tweet"),
        reply: false,
        retweet: false,
    };

    content_aggregator(tweet);
    // There's a new interesting content
    // Tweet by user1: this is my first tweet

    let news = NewsArticle {
        headline: String::from("Breaking News of Somewhere"),
        location: String::from("Somewhere"),
        author: String::from("Bond Man"),
        content: String::from("It's a secret news about Anya Forger"),
    };

    content_aggregator(news);
    // There's a new interesting content
    // News by Bond Man: It's a secret news about Anya Forger

    let contact = Contact {
        name: String::from("Some Name"),
        contact: String::from("0123456789"),
    };

    // Since we have default implementation for contact, we'll get default content
    content_aggregator(contact);
    // There's a new interesting content
    // Soemthing new... by Some Name
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarise(&self) -> String {
        format!("News by {}: {}", self.author, self.content)
    }
    fn get_author(&self) -> &str {
        self.author.as_str()
    }
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summary for Tweet {
    fn summarise(&self) -> String {
        format!("Tweet by {}: {}", self.username, self.content)
    }
    fn get_author(&self) -> &str {
        self.username.as_str()
    }
}

struct Contact {
    name: String,
    contact: String,
}
// Since we have default for summarize, we do not have to add the implementation here
// We only implement the get_author as it does not have any default implemetation
impl Summary for Contact {
    fn get_author(&self) -> &str {
        self.name.as_str()
    }
}

trait Summary {
    fn summarise(&self) -> String {
        // Since we cannot directly access the property of the structs implementing it, we can add
        // additional function called get_author which will give us the author
        format!("Something new... by {}", self.get_author())
    }

    fn get_author(&self) -> &str;
}

fn content_aggregator(source: impl Summary) {
    println!("There's a new interesting content");
    println!("{}", source.summarise());
}
