pub trait Summary {
    fn summarize(&self) -> String {
        let str = String::from("我是默认实现。");
        let str1 = String::from("调用了");
        let str2 = self.summarize_author();
        format!("{},{},{}", str, str1, str2)
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
        String::from("我是NewsArticle，我实现了Summary trait。")
    }
    fn summarize_author(&self) -> String {
        String::from("我是NewsArticle，我实现了Summary trait 的summarize_author方法。")
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        String::from("我是Tweet，我实现了Summary trait 的 summarize_author方法")
    }
}

pub fn notify(item: &impl Summary) {
    println!("我是notify。");
}

pub fn notify2<U: Summary>(item: &U, i2: &U) {
    println!("我是notify。");
}

pub fn notify3<U: Summary>(item: &U, i2: &U)
where
    U: Summary 
{
    println!("我是notify。");
}
