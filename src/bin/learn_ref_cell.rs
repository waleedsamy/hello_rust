#![allow(dead_code)]

trait Reporter {
    fn send(&self, message: &str) {
        println!("~~{}~~", message)
    }
}
struct RateLimiter<'a> {
    reporter: &'a dyn Reporter,
    max: i32,
    value: i32,
}

impl<'a> RateLimiter<'a> {
    fn new(reporter: &'a dyn Reporter, max: i32) -> Self {
        RateLimiter {
            reporter: reporter,
            max: max,
            value: 0,
        }
    }
    fn add_one(&mut self) {
        self.value += 1;
        let percent = self.value as f64 / self.max as f64;
        if percent > 1.0 {
            self.reporter.send("reach your quota")
        } else if percent > 0.9 {
            self.reporter.send("more than 90%")
        } else if percent > 0.75 {
            self.reporter.send("more than 75%")
        }
    }
}
fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockReporter {
        messages: Vec<String>,
    }

    impl MockReporter {
        fn new() -> Self {
            MockReporter { messages: vec![] }
        }
    }

    impl Reporter for MockReporter {
        fn send(&self, message: &str) {
            self.messages.push(message.to_string());
        }
    }

    #[test]
    fn send_message_when_reach_75_percent() {
        let mockReporter = MockReporter::new();
        let mut limiter = RateLimiter {
            reporter: &mockReporter,
            max: 100,
            value: 75,
        };
        limiter.add_one();

        assert_eq!(1, mockReporter.messages.len());
    }
}
