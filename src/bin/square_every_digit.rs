
fn square_digits(num: u64) -> u64 {
    num
        .to_string()
        .chars()
        .map(|i| i.to_digit(10).expect("char isnt digit").pow(2).to_string())
        .collect::<String>()
        .parse()
        .expect("result isnt u64 parsable")
}

fn main() {
    let res = square_digits(9999999999559);
    println!("{}", res);
}
