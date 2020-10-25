use rand::{thread_rng, Rng};
// use std::cmp::{max, min};
use std::io;

// use std::cmp::min;
// use std::cmp::max;

use std::{cmp::max, cmp::min};

fn main() {
    let l: i32 = min(6, 88);

    let _v1: Vec<i32> = Vec::new();

    let mut _v2 = std::vec::Vec::new();
    _v2.push(2);

    let _v3 = <Vec<i32>>::new();

    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    let _p1 = Point { x: 10, y: 15 };
    let _p2 = Point { x: 11, y: 156 };
    let _p3 = Point { x: 12, .._p2 };

    println!("_p1 {:?}, _p2 {:?}, _p3 {:#?}", _p1, _p2, _p3);
    let Point { x, y } = _p3;
    println!("x {}, y {}", x, y);

    struct Number {
        odd: bool,
        value: i32,
    }

    let _n1 = Number {
        odd: true,
        value: 1,
    };

    let _n2 = Number {
        odd: false,
        value: 2,
    };

    let _n3 = Number { value: 99, .._n2 };

    if let Number { odd: true, value } = _n1 {
        println!("{} is odd", value);
    } else if let Number { odd: true, value } = _n1 {
        println!("{} is even", value);
    }

    fn hh(n: Number) {
        match n {
            Number { value: 99, .. } => println!("is 99"),
            Number { odd: true, value } => println!("{} is odd", value),
            Number {
                odd: false,
                value: 55,
            } => println!("55 is odd"),
            Number { odd: false, value } => println!("{} is odd", value),
        }
    }

    hh(_n1);
    hh(_n2);
    hh(_n3);

    println!("Guess the number! {}", l);

    let mut trng = thread_rng();
    let secret_number: u8 = trng.gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let sum = |a: i32, b: i32| a + b;

    let n1: i32 = guess.trim().parse().unwrap();

    let f = sum(n1, n1);

    println!("You guessed: {} and sum: {}", guess, f);
}
