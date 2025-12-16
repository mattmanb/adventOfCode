use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::time::Instant;

fn main() -> io::Result<()> {
    let start_time = Instant::now();

    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut forklift: Vec<Vec<char>> = vec![];

    for line in reader.lines() {
        // Have the line (which is a string)
        //
        // Split each line into a row in a 2D vector
        let char_vec: Vec<char> = line.unwrap().chars().collect();
        forklift.push(char_vec.clone());

        println!("{:?}", char_vec);
    }



    Ok(())
    
}

