#![allow(unused_variables)]

struct Point {
    x: u8,
    y: u8,
}

fn main() {
    let v = vec!['a', 'b', 'c'];

    for (i, v) in v.iter().enumerate() {
        println!("{} at index {}", v, i);
    }

    let (x, y, z) = (1, 2, 3);
    let (x, y, _) = (4, 5, 6);
    assert_eq!(x, 4);
    assert_eq!(y, 5);
    assert_eq!(z, 3);

    let p1 = Point { x: 10, y: 5 };
    let Point { x: p1x, y: p1y } = p1;
    assert_eq!(p1x, 10);
    assert_eq!(p1y, 5);

    let nums = (1, 2, 3, 4, 5);
    match nums {
        (first, .., last) => println!("first {}, last {}", first, last),
    }
}
