
fn find_short(s: &str) -> u32 {
    s.split(" ").map(|x| x.len()).min().unwrap() as u32
}

fn main() {
    let shortest = find_short(&"this is a test sentence");
    println!("{}", shortest);
}
