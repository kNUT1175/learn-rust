use traits::{NewsArticle, SocialPost, Summary};

fn main() {
    let post1 = SocialPost { username: "aknutson".to_string(), content: "I love learning Rust".to_string(), reply: false, repost: false };
    println!("{}\n", post1.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship"),
        location: String::from("Pittsburgh, PA"),
        author: String::from("Ryan Divish"),
        content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
    };

    println!("New article available: {}\n", article.summarize());
    
    let post = SocialPost {
        username: "aknutson".to_string(),
        content: "I'm such a fat fucking chud!".to_string(),
        reply: false,
        repost: false,
    };
    println!("New post from aknutson available: {}", post.summarize());



}
