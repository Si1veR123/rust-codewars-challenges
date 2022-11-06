fn josephus_survivor(n: i32, k: i32) -> i32 {
    let mut count = 0;

    let mut survivors = (1..n+1).collect::<Vec<i32>>();

    let mut length = survivors.len() as i32;

    for _round in 0..n-1 {
        count = (count+k-1)%(length);
        length -= 1;
        survivors.remove(count as usize);
    }
    survivors[0]
}

fn main() {
    let survivor = josephus_survivor(7, 3);
    println!("{}", survivor);
}
