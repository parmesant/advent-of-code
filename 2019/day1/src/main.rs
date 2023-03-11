fn main() {
    // read the input masses from input.txt file
    let binding = std::fs::read_to_string("input.txt")
        .unwrap();
    
    let mut masses: Vec<i64> = binding
        .split("\n")
        .filter_map(|v| {
            if v.len()>0 {
                Some(v.parse::<i64>().unwrap())
            } else {
                None
            }
        })
        .collect();
    let sum = partTwo(&mut masses);
    println!("{sum}");
}

fn partOne(masses: &Vec<i64>) -> i64 {
    let mut sum: i64 = 0;

    masses.iter()
        .for_each(|mass| {
            sum+=((mass/3)-2);
        });
    sum
}

fn partTwo(masses: &mut Vec<i64>) -> i64 {
    let mut sum: i64 = 0;

    masses.iter_mut()
        .for_each(|mass| {
            let mut m = mass.clone();
            while m>0 {
                let mut fuel = (m/3)-2;
                if fuel<0 {
                    fuel = 0;
                }
                sum+=fuel;
                m = fuel;
            }
        });
    sum
}