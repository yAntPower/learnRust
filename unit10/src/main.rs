use unit10::*;
fn main() {
    let tw = Tweet {
        username: String::from("name"),
        content: String::from("content"),
        reply: false,
        retweet: false,
    };
    println!("{}", tw.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
         hockey team in the NHL.",
        ),
    };
    println!("{}", article.summarize());
    notify(&article);
    notify(&tw);

}
