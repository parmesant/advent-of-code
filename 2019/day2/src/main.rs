fn main() {
    // get the inputs
    let binding = std::fs::read_to_string("input.txt").unwrap();

    let mut input_vec: Vec<i64> = binding
        .split("\n")
        .filter_map(|v| {
            if v.len()>0 {
                Some(v.parse::<i64>().unwrap())
            } else {
                None
            }
        })
        .collect();


}

fn partOne(input_vec: &mut Vec<i64>) {
    
}