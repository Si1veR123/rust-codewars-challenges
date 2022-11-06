
fn diamond_size_vector(n: u32) -> Vec<u32> {
    let mut increasing = (1..n+1).step_by(2).collect::<Vec<u32>>();
    let mut decreasing = increasing.clone();
    decreasing.reverse();
    increasing.extend_from_slice(&decreasing[1..]);
    return increasing
}

fn star_string_from_vec(vector: &Vec<u32>) -> String {
    let max = vector.iter().max().unwrap();
    let mut running_string = String::from("");
    for n in vector {
        running_string.extend(" ".repeat(((max-*n)/2) as usize).chars());
        running_string.extend("*".repeat(*n as usize).chars());
        running_string.extend("\n".chars());
    }
    running_string
}

fn main() {
    let diamond = diamond_size_vector(20);
    let output_string = star_string_from_vec(&diamond);
    println!("{}", output_string);
}
