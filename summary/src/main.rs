fn main() {
    let news = News {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
    };
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
    };

    summary(news);
    summary(tweet);
}

fn summary<T: Summarizable>(t: T) {
    println!("summary: {}", t.summarize());
}

trait Summarizable {
    fn summarize(&self) -> String;
}

struct News {
    headline: String,
    location: String,
    author: String,
}

impl Summarizable for News {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

struct Tweet {
    username: String,
    content: String,
}

impl Summarizable for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
