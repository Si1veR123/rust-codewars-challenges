
fn alphabet_position(text: &str) -> String {
    let ascii_numbers = text
        .to_lowercase()
        .chars()
        .filter(|x| x.is_ascii_alphabetic())
        .map(|x| ((x as i16)-96).to_string() + " ")
        .collect::<Vec<String>>();
    
    if ascii_numbers.is_empty() {
        return String::new()
    }

    let string = String::from_iter(ascii_numbers);
    string[..string.len()-1].to_string()
}

fn main() {
    alphabet_position("abcc");
}
