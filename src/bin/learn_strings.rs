use unicode_segmentation::UnicodeSegmentation;
fn main() {
    let mut s = String::new();
    s.push_str("foo");
    println!("{}", s);

    let data = "initial contents";
    let mut s = data.to_string();
    s.push(' ');
    s.push_str("Olá");
    println!("{}", s);

    let s1 = String::from("hello");
    let s2 = String::from(" world");

    let s3: String = s1 + &s2;
    // println!("s1 {}", s1); // s1 moved
    println!("s2 {}", s2);
    println!("s3 {}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s4 = s1 + " " + &s2 + " " + &s3;
    println!("s4 {}", s4);
    let s1 = String::from("tic"); // prev s1 is moved to s4
    let s4 = format!("{} {} {}", s1, s2, s3);
    println!("s4 {}", s4);

    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("{}", s);

    // let s = &hello[0..1];
    // println!("{}", s); // panic at runtime, byte index 1 is not a char boundary

    let s = "नमस्ते";
    for c in s.chars() {
        // print characters
        println!("{} char {}", s, c);
    }

    let g = UnicodeSegmentation::graphemes(s, true).collect::<Vec<&str>>();
    for grapheme in &g {
        //print graphemes
        println!("{} grapheme {}", s, grapheme);
    }
}
