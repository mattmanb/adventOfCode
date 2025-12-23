use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::time::Instant;

fn main() -> io::Result<()> {
    // Start timer
    let start = Instant::now();

    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut valid_ids: Vec<(u64, u64)> = Vec::new();
    let mut ids: Vec<u64> = Vec::new();

    let mut total_valid_ids: u64 = 0;

    // Loop over each line
    for line in reader.lines() {
        let is_range = line.as_ref().unwrap().find('-');

        if is_range.is_none() {
            let ingredient_id = line.unwrap().clone();
            if ingredient_id.len() == 0 {
                continue;
            }
            let num: u64 = ingredient_id.parse().expect("Failed to parse ingredient ID");
            // Need a vector of ids to store the parsed number in
            ids.push(num);
        } else {
            let range = line.unwrap().clone();
            let range_separator = is_range.unwrap();
            let lower_str = &range[..range_separator];
            let upper_str = &range[range_separator+1..];
            let lower: u64 = lower_str.parse().expect("Failed to parse lower bound");
            let upper: u64 = upper_str.parse().expect("Failed to parse upper bound");
            valid_ids.push((lower, upper));
            total_valid_ids += upper-lower+1;
        }
    }
    let mut total_valid: u32 = 0;
    for id in ids {
        for (lower, upper) in &valid_ids {
            if id >= *lower && id <= *upper {
                total_valid += 1;
                break;
            }
        }
    }

    // part 2

    for i in 0..valid_ids.len()-1 {
        for j in i+1..valid_id.len() {

        }
    }

    let elapsed = start.elapsed();

    println!("Execution time (seconds): {:?}", elapsed.as_secs_f32());
    println!("Fresh inventory: {}", total_valid);
    println!("Total fresh inventory: {}", total_valid_ids);

    Ok(())


}
