use std::fmt;

#[derive(Debug)]
struct Rectangle {
    x: i32,
    y: i32,
}

impl Rectangle {
    fn area(&self) -> i32 {
        self.x * self.y
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.x < other.x && self.y < other.y
    }

    fn sqaure(x: i32) -> Rectangle {
        Rectangle { x: x, y: x }
    }
}

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

    let rec1 = Rectangle { x: 7, y: 8 };
    let rec2 = Rectangle { x: 10, y: 15 };

    let res2_area = rec2.area();
    println!("rec2 {:?} has area {}", rec2, res2_area);

    let res2_area = area(&rec2);
    println!("rec2 {:?} has area {}", rec2, res2_area);

    println!(
        "rec1 {:?} has area {} can hold rec1 {:?} ? {}",
        rec2,
        res2_area,
        rec1,
        rec1.can_hold(&rec2)
    );

    let sq_10 = Rectangle::sqaure(10);
    println!("sq_10 {:?} has area {}", sq_10, sq_10.area());
}

fn area(rec: &Rectangle) -> i32 {
    return rec.x * rec.y;
}
