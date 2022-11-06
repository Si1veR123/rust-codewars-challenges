fn sequence_sum(start: u32, end: u32, step: u32) -> u32 {
    // O(1)
    if start > end {return 0}

    let new_start;
    let mut prev_height: u32 = 0;
    match start {
        0 => {new_start = step; if step>1 {prev_height = new_start-1;}},
        1 => new_start = start,
        _ => {new_start = start; prev_height = new_start-1;}
    }

    let n = ((end-new_start+1) as f32/step as f32).ceil() as u32;

    (step*n*(n+1))/2 - n*(step-1) + prev_height*n
}


fn main() {
    println!("{}", sequence_sum(0, 15, 3))
}
