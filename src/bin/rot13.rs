
fn shift_char(c: char) -> char {
    if !c.is_ascii_alphabetic() {return c}
    if c.is_lowercase() {
        char::from_u32((((c as u32)-83)%25)+96).unwrap()
    } else {
        char::from_u32((((c as u32)-51)%25)+64).unwrap()
    }
}

fn rot13(message: &str) -> String {
    message.chars()
        .map(|c| shift_char(c))
        .collect::<String>()
}

fn main() {
    let s = rot13("m");
    println!("{}", s);
}
