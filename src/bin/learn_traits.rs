use std::fmt::{Debug, Display, Formatter, Result};
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("Read more by {}...", self.summarize_author())
    }
}

#[derive(Debug)]
pub struct NewArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

#[derive(Debug, Clone)]
pub struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summary for NewArticle {
    fn summarize_author(&self) -> String {
        format!("by {}", self.author)
    }
}

impl Display for NewArticle {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.summarize())
    }
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    fn summarize(&self) -> String {
        if self.retweet {
            format!("{} retweeted:{}", self.username, self.content)
        } else if self.reply {
            format!("{} replied:{}", self.username, self.content)
        } else {
            format!("{} tweeted:{}", self.username, self.content)
        }
    }
}

impl Display for Tweet {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.summarize())
    }
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Pair<T> {
        Pair { x: x, y: y }
    }
}

impl<T: PartialOrd + Display> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn main() {
    let article_1 = NewArticle {
        headline: "Terror attack in Vienna leaves four dead".to_string(),
        location: "Vienna".to_string(),
        author: "CNN".to_string(),
        content: "....".to_string(),
    };

    notify(&article_1);
    notify_1(&article_1);

    let tweet_1 = Tweet {
        username: "AP".to_string(),
        content:"Austria’s top security official says five people have died and 17 others were wounded in the shooting.".to_string(),
        retweet: false,
        reply: false,
    };

    notify(&tweet_1);
    notify_1(&tweet_1);

    let p = Pair { x: 10, y: 10 };
    p.cmp_display();

    let p = Pair {
        y: tweet_1.clone(),
        x: tweet_1,
    };
    // p.cmp_display(); // Tweet doesn't satisfy `Tweet: std::cmp::PartialOrd` so cmp_display not available
}

fn notify(summeriable: &(impl Summary + Display)) {
    println!("notify -> {}", summeriable.summarize());
}

fn notify_1<T: Summary + Display>(summeriable: &T) {
    println!("notify_1 -> {}", summeriable.summarize());
}

fn some_fun_1<T: Display + Clone, U: Debug + Clone>(t: T, u: U) {}
fn som_fun_2<T, U>(t: T, u: U)
where
    T: Display + Clone,
    U: Debug + Clone,
{
}

fn returns_summarizable(b: bool) -> impl Summary {
    Tweet {
        username: "AP".to_string(),
        content:"Austria’s top security official says five people have died and 17 others were wounded in the shooting.".to_string(),
        retweet: false,
        reply: false,
    }
}

mod tests {
    use crate::{Summary, Tweet};
    #[test]
    fn test_notify() {
        let tweet_1 = Tweet {
            username: "AP".to_string(),
            content:"Austria’s top security official says five people have died and 17 others were wounded in the shooting.".to_string(),
            retweet: false,
            reply: false,
        };

        assert_eq!("AP tweeted:Austria’s top security official says five people have died and 17 others were wounded in the shooting.", tweet_1.summarize())
    }
}
