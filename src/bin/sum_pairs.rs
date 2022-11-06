
fn sum_of_pairs(vector: Vec<i16>, sum: i16) -> (i16, i16) {
    // O(n^2)
    for x in &vector {
        for y in &vector {
            if x+y == sum {
                return (*x, *y)
            }
        }
    }
    return (0, 0)
}

fn main() {
    let mut vector_ints: Vec<i16> = Vec::new();
    vector_ints.push(3);
    vector_ints.push(5);
    vector_ints.push(2);
    vector_ints.push(10);
    vector_ints.push(1);
    vector_ints.push(-5);
    vector_ints.push(15);
    vector_ints.push(20);
    
    let pair: (i16, i16) = sum_of_pairs(vector_ints, -4);
    println!("{}, {}", pair.0, pair.1)
}
