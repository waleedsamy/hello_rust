use std::fmt;

struct Wrapper<T: fmt::Display>(Vec<T>);

impl<T: fmt::Display> fmt::Display for Wrapper<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "vec![{}]",
            self.0
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

impl<T: fmt::Display> std::ops::Deref for Wrapper<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct Point {
    x: u8,
    y: u8,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Point({}, {})", self.x, self.y)
    }
}

fn main() {
    let v = Wrapper(vec![1, 2, 3]);
    println!("{}, length={}", v, v.len());

    let v = Wrapper(vec!['a', 'b', 'c']);
    println!("{}, length={}", v, v.len());

    let v = Wrapper(vec![Point { x: 1, y: 3 }, Point { x: 10, y: 17 }]);
    println!("{}, length={}", v, v.len());

    assert_eq!(Some("10".to_string()), map(to_str, Some(10)));
    assert_eq!(None, map(to_str, None));
}

fn to_str(i: i32) -> String {
    format!("{}", i)
}

fn map<T, U>(op: fn(T) -> U, v: Option<T>) -> Option<U> {
    match v {
        Some(inner) => Some(op(inner)),
        None => None,
    }
}
