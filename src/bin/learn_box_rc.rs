use std::cell::RefCell;
use std::rc::Rc;
#[derive(Debug)]
enum List<T> {
    Cons(Rc<RefCell<T>>, Rc<List<T>>),
    Nil,
}

impl<T> List<T> {
    fn create(elem: T) -> Rc<RefCell<T>> {
        Rc::new(RefCell::new(elem))
    }
}

fn main() {
    use self::List::{Cons, Nil};

    let b = Box::new(10);
    println!("b= {} ({:p})", b, &b);

    let value = List::create(5); // owned by list_a too

    let list_a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
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

    let list_b = Cons(List::create(3), Rc::clone(&list_a));
    println!("count of owner of list_a {}", Rc::strong_count(&list_a));
    {
        let _list_c = Cons(List::create(4), Rc::clone(&list_a));
        println!("count of owner of list_a {}", Rc::strong_count(&list_a));
    }
    let list_d = Cons(List::create(4), Rc::clone(&list_a));
    println!("count of owner of list_a {}", Rc::strong_count(&list_a));

    *value.borrow_mut() += 10;

    println!("list_a= {:?} ({:p})", list_a, &list_a);
    println!("list_b= {:?} ({:p})", list_b, &list_b);
    println!("list_d= {:?} ({:p})", list_d, &list_d);
}
