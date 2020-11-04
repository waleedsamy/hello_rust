use std::fmt::Display;
fn main() {
    let x = "a";
    let y = "b";
    println!("{} is longer", choose_ref(x, y));
    println!("{} is longer", longest_with_an_announcement(x, y, y));
}

fn choose_ref<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest_with_an_announcement<'a, T: Display>(x: &'a str, y: &'a str, ann: T) -> &'a str {
    println!("note: {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
