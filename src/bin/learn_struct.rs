use std::fmt;

fn main() {
    #[derive(Debug)]
    struct Number {
        odd: bool,
        value: i32,
    }

    impl Number {
        fn is_strictly_positive(self) -> bool {
            self.value > 0
        }
    }

    impl fmt::Display for Number {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "(odd={}, value={})", self.odd, self.value)
        }
    }

    let _n1 = Number {
        odd: true,
        value: 1,
    };

    match _n1.is_strictly_positive() {
        true => println!("is postive"),
        false => println!("is negative or zero"),
    }
}
