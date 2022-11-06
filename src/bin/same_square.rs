
fn same_squares(a: Vec<i64>, b: Vec<i64>) -> bool {
    let mut b = b.clone();
    for square in a {
        let squared = i64::pow(square, 2);
        if !b.contains(&squared) {
            return false
        }
        b.remove(b.iter().position(|x| *x == squared).unwrap() as usize);
    }
    true
}

fn main() {
    let a = vec![121, 144, 19, 161, 19, 144, 19, 11];
    let b = vec![121, 14641, 20736, 361, 25921, 361, 20736, 361];
    let same = same_squares(a, b);
    println!("{}", same);
}
