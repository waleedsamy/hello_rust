#[derive(Debug)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}
fn main() {
    use self::List::{Cons, Nil};

    let b = Box::new(10);
    println!("b= {} ({:p})", b, &b);

    let list1 = Cons(1, Box::new(Cons(2, Box::new(Nil))));
    println!("list1= {:?} ({:p})", list1, &list1);

    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x = 5;
    let y = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);
}
