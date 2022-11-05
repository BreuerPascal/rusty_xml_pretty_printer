fn main() {
    println!("Hello, world!");
    let s = "abcäöü€";
    s.chars().for_each(|c| println!("c: {}", c));
}

fn p(xml: &str) -> &str {
    p(&xml[1..])
}
