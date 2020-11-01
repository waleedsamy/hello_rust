#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

impl Point<f32, f32> {
    fn points_of_f32(&self) -> f32 {
        self.x + self.y
    }
}

fn main() {
    let v_1 = vec![10, 20, 30, 20];
    println!("elem {} is the largest in {:?}", largest(&v_1), v_1);

    let v_2 = vec![10.5, 20.1, 30.33, 20.22];
    println!("elem {} is the largest in {:?}", largest(&v_2), v_2);

    let v_3: Vec<char> = "When we use a parameter".chars().collect();
    println!("elem {} is the largest in {:?}", largest(&v_3), v_3);

    let point_of_int = Point {
        x: 10_u32,
        y: 20_u32,
    };
    // point_of_int.points_of_f32(); // points_of_f32 not defined for Point<u32, u32>
    println!("point_of_int: {:?}", point_of_int);

    let point_of_f32 = Point {
        x: 10.0_f32,
        y: 20.0_f32,
    };
    point_of_f32.points_of_f32();
    println!("point_of_f32: {:?}", point_of_f32);

    let mixed = point_of_int.mixup(point_of_f32);
    println!("mixed: {:?}", mixed);
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for elem in list {
        if elem > largest {
            largest = elem;
        }
    }
    largest
}
