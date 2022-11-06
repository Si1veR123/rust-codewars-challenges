
fn smallest(n: i64) -> (i64, usize, usize) {
    let min = n.to_string()[1..].chars().min().expect("no min");
    let min_pos = n.to_string().chars().position(|x| x == min).expect("min not found");
    let mut final_num = n.to_string();
    final_num.remove(min_pos);
    
    let mut insert_index = 0;
    for (index, num) in final_num.chars().enumerate() {

        if num.to_digit(10).unwrap() > min.to_digit(10).unwrap() {
            insert_index = index;
            break;
        }
    }
    final_num.insert(insert_index, min);
    
    if min == '0' {
        (final_num.parse().expect("number"), insert_index, (min_pos) as usize)
    } else {
        (final_num.parse().expect("number"), (min_pos) as usize, insert_index)
    }
}

fn main() {
    let small = smallest(285365);
    println!("{}, {}, {}", small.0, small.1, small.2);
}
