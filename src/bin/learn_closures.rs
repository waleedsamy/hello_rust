use std::collections::HashMap;
struct Cacher<T>
where
    T: Fn(i32) -> i32,
{
    calculation: T,
    _map: HashMap<i32, i32>,
}

impl<T> Cacher<T>
where
    T: Fn(i32) -> i32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation: calculation,
            _map: HashMap::new(),
        }
    }

    fn value(&mut self, arg: i32) -> i32 {
        match self._map.get(&arg) {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(arg);
                self._map.insert(arg, v);
                v
            }
        }
    }
}

fn main() {
    let mut cacher = Cacher::new(|num| {
        println!("call once: {}", num);
        num
    });

    cacher.value(33);
    cacher.value(33);
    cacher.value(33);
}
