
fn main() {
    println!("Hello, world!");

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let result = largest_generic(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_generic(&char_list);
    println!("The largest char is {}", result);


    // generic struct
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}, p.y = {}", p.x(), p.y());

    // trait
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());
    println!("Author: {}", article.summarize_author());

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
    println!("Author: {}", tweet.summarize_author());

    // trait as parameter
    notify(&article);
    notify(&tweet);

    // trait as parameter with multiple bounds
    let s1 = String::from("long string is long");
    let s2 = "xyz";

    let result = longest(s1.as_str(), s2);
    println!("The longest string is \"{}\"", result);

    let i1 = ImportantExcerpt {
        part: "I really like this book",
    };

    println!("Level: {}", i1.level());
    println!("Part: {}", i1.annouce_and_return_part());

}

fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for item in list.iter() {
        if *item > largest {
            largest = *item;
        }
    }
    largest
}

fn largest_generic<T>(list: &[T]) -> &T
where
    T: PartialOrd,
{
    let mut largest = &list[0];
    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &T {
        &self.y
    }
}

pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
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
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
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
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// trait as parameter
fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
    fn annouce_and_return_part(&self) -> &'a str {
        self.part
    }
}

