
pub struct NewArticle {
    pub headline :String,
    pub location :String,
    pub author :String,
    pub content :String,
}

impl  Summary for NewArticle {
    fn summarize_author(&self) -> String {
        format!("{}",  self.author)
    }
}

pub struct Tweet {
    pub username :String,
    pub content :String,
    pub reply :bool,
    pub retweet :bool,
}

impl  Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String{
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub fn notify (item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

fn main() {
    let tweet = Tweet {
        username: String::from("Uchenna"),
        content: String::from("Hello world"),
        reply: false,
        retweet: false,
    };


    let article = NewArticle {
        author: String::from("Uche Doe"),
        headline: String::from("Understanding Rust "),
        location: String::from("Awka "),
        content: String::from("God has made me wiser than my teachers"),
    };

    notify(&article);

    // println!("Tweet summary: {}", tweet.summarize());
    // println!("Article summary: {}", article.summarize());
}