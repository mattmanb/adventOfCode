use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::time::Instant;

fn main() -> io::Result<()> {
    // Start timer
    let start = Instant::now();

    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut sum = 0;
    let mut sum2: u128 = 0;

    // Loop over each line
    for line in reader.lines() {
        let is_range = line.unwrap().find('-');

        if is_range == None {
            let ingredient_id = line.clone().unwrap();
            let num: u128 = ingredient_id.parse().expect("Failed to parse ingredient ID");
        } else {
            let range_separator = is_range.unwrap();
        }
    }

    Ok(())
}
