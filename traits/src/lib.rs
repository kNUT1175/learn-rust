// pub trait Summary {
//     fn summarize(&self) -> String;
// }


pub trait Summary {
    fn summarize(&self) -> String{
        format!("Read more from {}...", self.summarize_author())
    }

    fn summarize_author(&self) -> String;
}


pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({}) \n {}", self.headline, self.author, self.location, self.summarize_author())
    }
    fn summarize_author(&self) -> String {
        format!("{} has been covering the penguins for 67 years", self.author)
    }
}
// impl Summary for NewsArticle {}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.summarize_author(), self.content)
    }
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}


pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}