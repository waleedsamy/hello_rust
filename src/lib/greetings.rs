#![allow(dead_code)]

#[derive(Debug)]
pub struct Rectangle {
    pub width: i32,
    pub hight: i32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.hight > other.hight
    }
}

pub fn greeting(name: &str) -> String {
    let mut s = String::from("Hello!");
    s.push_str(" ");
    s.push_str(name);
    s
}

#[allow(dead_code)]
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(i: i32) -> Guess {
        if i < 1 {
            panic!("Guess must be more than 1, got {}", i);
        } else if i > 100 {
            panic!("Guess must be less than 100, got {}", i);
        } else {
            Guess { value: i }
        }
    }

    pub fn maybe_new(i: i32) -> Result<Guess, String> {
        if i < 1 {
            Err(format!("Guess must be more than 1, got {}", i))
        } else if i > 100 {
            Err(format!("Guess must be less than 100, got {}", i))
        } else {
            Ok(Guess { value: i })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{greeting, Guess, Rectangle};
    #[test]
    fn it_works() {
        assert_eq!(4, 2 + 2);
    }
    #[test]
    fn test_larger_can_hold() {
        let rec1 = Rectangle {
            width: 10,
            hight: 20,
        };

        let rec2 = Rectangle {
            width: 6,
            hight: 10,
        };

        assert!(rec1.can_hold(&rec2));
    }
    #[test]
    fn test_smaller_can_not_hold() {
        let rec1 = Rectangle {
            width: 10,
            hight: 20,
        };

        let rec2 = Rectangle {
            width: 6,
            hight: 10,
        };

        assert!(!rec2.can_hold(&rec1));
    }

    #[test]
    fn test_greeting() {
        let hello_world = greeting("world");
        assert!(
            hello_world.contains("world"),
            "{} not contain 'world'",
            hello_world
        );
    }

    #[test]
    #[should_panic(expected = "Guess must be more than 1, got -1")]
    fn test_guess_must_be_more_than_1() {
        Guess::new(-1);
    }

    #[test]
    #[should_panic(expected = "Guess must be less than 100, got 200")]
    fn test_guess_must_be_less_than_100() {
        Guess::new(200);
    }

    #[test]
    fn test_guess() -> Result<(), String> {
        Guess::maybe_new(10).map(|_| ())
    }

    // cargo test --lib -- --ignored
    // cargo test --lib -- --ignored ignored_1
    #[test]
    #[ignore = "....."]
    fn test_ignored_1() {}

    #[test]
    #[ignore = "....."]
    fn test_ignored_2() {}
}
