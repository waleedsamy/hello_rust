fn main() {
    let mut _v1: Vec<i16> = Vec::new();
    _v1.push(1);
    _v1.push(2);
    _v1.push(3);

    let second = &_v1[1];
    let thrid = &_v1[2];
    println!("second: {:p} {}", second, second);
    println!("thrid: {:p} {}", thrid, thrid);
    println!("_v1: {:?}", _v1);
    // _v1.push(8); // panic, can have mutable and immutable ref (next line) at same time
    println!("thrid: {:p} {}", thrid, thrid);

    match _v1.get(2) {
        Some(n) => println!("index 2 has: {}", n),
        None => println!("out of index"),
    }

    let _v1 = vec![1, 2, 3];
    println!("_v1: {:?}", _v1);

    let v = vec![1, 2, 3, 4, 5];
    // let _thrid = &v[32]; // panic
    let _thrid = v.get(32); // no panic

    for i in &v {
        println!("{}", i)
    }

    let mut v = vec![1, 2, 3, 4, 5];
    for i in &mut v {
        *i = *i * 10;
    }
    println!("v mul by 10 {:?}", v);

    #[derive(Debug)]
    enum SpreadSheet {
        Int(i32),
        Float(f32),
        Text(String),
    };

    let _v: Vec<SpreadSheet> = vec![
        SpreadSheet::Float(33.0),
        SpreadSheet::Int(5),
        SpreadSheet::Text("hello".to_string()),
    ];
    for i in &_v {
        println!("_v: {:p} {:?}", i, *i);
    }

    let mut vec = Vec::new();
    vec.push(9);
    vec.extend([1, 2, 3].iter());

    for i in &vec {
        println!("vec: {:p} {:?}", i, *i);
    }
}
