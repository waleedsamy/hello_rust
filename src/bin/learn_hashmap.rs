use std::collections::HashMap;

type Int = i32;

fn main() {
    let mut m: HashMap<String, Int> = HashMap::new();
    let team_blue = String::from("blue");
    let team_red = String::from("red");
    m.insert(team_red, 10);
    m.insert(team_blue, 5);
    m.insert(String::from("blue"), 99);
    println!("m: {:?}", m);

    let names = vec![String::from("red"), String::from("blue")];
    let ranks = vec![10, 5, 11];

    let mut m: HashMap<_, _> = names.into_iter().zip(ranks.into_iter()).collect();
    println!("m: {:?}", m);

    let blue_score = m.get("red");
    println!("blue_score: {:?}", blue_score);

    for (key, value) in &m {
        println!("{} -> {}", key, value);
    }

    let red_score = m.entry(String::from("red")).or_insert(999);
    println!("red score: {:?}", red_score);

    let black_score = m.entry(String::from("black")).or_insert(999);
    println!("red score: {:?}", black_score);

    println!("m: {:?}", m);

    let mut words_count: HashMap<String, Int> = HashMap::new();
    let text = "hello world wonderful world";

    for word in text.split(' ') {
        let count = words_count.entry(word.to_string()).or_insert(0);
        *count += 1;
    }
    println!("words_count: {:?}", words_count);
}
