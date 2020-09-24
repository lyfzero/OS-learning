fn main() {
    // 定义
    pub trait Summary {
        fn summarize(&self) -> String;
        fn note(&self) -> String {
            String::from("(Read more...)")
        }
    }
    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }
    impl Summary for NewsArticle {   // 实例化
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }
    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }
    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already "),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
    println!("1 new tweet: {}", tweet.note());

    // 作为参数
    pub fn notify(item: impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }
    // trait bound

    let number_list = vec![12, 2, 54, 62];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['s', 'a', 'f', 'h'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);

}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

