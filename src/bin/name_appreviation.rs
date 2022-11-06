fn abbrev_name(name: &str) -> String {
    name.to_string().split(" ")
        .map(|x| x.chars().next().unwrap().to_string().to_uppercase())
        .collect::<Vec<String>>()
        .join(".")
}

fn main() {
    println!("{}", abbrev_name("Sam Harris"))
}
