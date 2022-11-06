fn find_missing_letter(chars: &[char]) -> char {
    let upper = chars.get(0).expect("not empty").is_uppercase();

    let mut alphabet = String::from("abcdefghijklmnopqrstuvwxyz");
    if upper {
        alphabet.make_ascii_uppercase();
    }

    let first_index = alphabet
        .chars()
        .position(|x| x==chars[0])
        .expect("Letter not in alphabet");

    let expected = &alphabet
        [first_index..first_index+chars.len()]
        .chars()
        .find(|x| !chars.contains(x))
        .expect("none missing");

    *expected
}

fn main() {
    let letter = find_missing_letter(&['B', 'C', 'D', 'F']);
    println!("{}", letter);
}
