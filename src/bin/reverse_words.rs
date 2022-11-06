use std::future::join;


fn reverse_words(str: &str) -> String {
    str
        .split(" ")
        .map(|x| x.chars().rev().collect::<String>())
        .collect::<Vec<String>>()
        .join(" ")
        .to_string()
}
fn main() {

}
