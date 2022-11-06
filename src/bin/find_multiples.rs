fn find_multiples(n: u32, limit: u32) -> Vec<u32> {
    (1..(limit+1)).filter(|x| x%n == 0).collect()
}

fn main() {
    let v = find_multiples(1, 2);
    println!("{}", v[0]);
}
