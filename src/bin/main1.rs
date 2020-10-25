const MAX_POINT: u32 = 1_000;

#[derive(Debug)]
enum Coin {
    Penny,
    Cent,
    Dime,
}

struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u32,
}

struct Color(u32, u32, u32);

fn main() {
    let mut x = 10;
    println!("x is {}", x);
    x = 11;
    println!("x is {}", x);
    println!("MAX_POINT is {}", MAX_POINT);

    let x = x as f32 + 11.03;

    println!("shadow x is {}", x);

    let guess: i32 = "42".parse().expect("excpedt int");
    println!("guess {}", guess);

    let x: bool = true;
    println!("x is {}", x);

    let x: (u8, u8, u16) = (b'A', 44, 1000);
    let (a, b, c) = x;
    println!("x is {:?}", x);
    println!("xa={},xb={},xc={}", a, b, c);
    println!("xa={},xb={},xc={}", x.0, x.1, x.2);

    let x = [1, 2, 3, 4];
    println!("x is {:?}", x);
    let x: [i32; 4] = [1, 2, 3, 4];
    println!("x is {:?}", x);
    let x = [3; 5];
    println!("x is {:?}", x);
    let x = another_function(7);
    println!("x from another_function() is {}", x);

    let x = if x > 20 {
        println!("{} > 20", x);
        x + 1
    } else if x > 10 {
        println!("{} > 10", x);
        x + 1
    } else {
        println!("{} > 0", x);
        x + 1
    };
    println!("x is {:}", x);

    let x = [1, 2, 3, 4, 5, 6, 7];
    let mut index = 0;
    while index < x.len() {
        println!("elem at index {} is {}", index, x[index]);
        index += 1;
    }
    let x = 1..5;
    for e in (1..5).rev() {
        println!("elem {} is", e);
    }

    for e in 1..3 {
        println!("e is {}", e)
    }

    #[derive(Debug)]
    enum IPAddrKind {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home: IPAddrKind = IPAddrKind::V4(172, 0, 0, 1);
    let loopback = IPAddrKind::V6(String::from("::1"));

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColur(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {}
    }

    let x = Message::Quit;
    let x = Message::Move { x: 10, y: 11 };
    let x = Message::Write(String::from("abc"));
    let x = Message::ChangeColur(1, 2, 3);

    let x = Coin::Penny;
    println!("coin {:?} has {}", x, coin_to_u8(&x));

    let some_u8_value = Some(3u8);
    match some_u8_value {
        Some(3) => println!("three..."),
        _ => (),
    }

    if let Some(3) = some_u8_value {
        println!("three...")
    }

    let mut x = build_user("xx@mail.com".to_string(), "xx".to_string());

    println!("x username: {}", x.username);
    x.username = String::from(format!("{}..updated", x.username));
    println!("x updated username: {}", x.username);

    let x = Color(1, 2, 3);
    println!("color a {}, b {}, c {}", x.0, x.1, x.2);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 0,
    }
}

fn coin_to_u8(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        _ => 2,
    }
}

fn another_function(i: i32) -> i32 {
    i + 11
}

fn inc(x: usize) -> usize {
    x + 1
}

fn inc_ref(x: &usize) -> usize {
    let address = format!("{:p}", x);
    println!("{}", address);
    x + 1
}
