use std::rc::Rc;
#[derive(Debug)]
enum List<T> {
    Cons(T, Rc<List<T>>),
    Nil,
}
fn main() {
    use self::List::{Cons, Nil};

    let b = Box::new(10);
    println!("b= {} ({:p})", b, &b);

    let list_a: Rc<List<i32>> = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("list_a= {:?} ({:p})", list_a, &list_a);
    println!("count of owner of list_a {}", Rc::strong_count(&list_a));

    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x = 5;
    let y = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    let _list_b = Cons(3, Rc::clone(&list_a));
    println!("count of owner of list_a {}", Rc::strong_count(&list_a));
    {
        let _list_c = Cons(4, Rc::clone(&list_a));
        println!("count of owner of list_a {}", Rc::strong_count(&list_a));
    }
    let _list_d = Cons(4, Rc::clone(&list_a));
    println!("count of owner of list_a {}", Rc::strong_count(&list_a));
}
