
fn fibbonacci(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibbonacci(n-1) + fibbonacci(n-2),
    }
}

fn perimeter(n: u32) -> u32 {
    4*(fibbonacci(n+2)-1)
}

fn main() {
    let perim = perimeter(30);
    println!("{}", perim);
}