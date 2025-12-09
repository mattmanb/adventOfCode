// use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::time::Instant;

fn main() -> io::Result<()> {
    // Start timer
    let start = Instant::now();
    // Read file input
    // let args: Vec<String> = env::args().collect();
    // if args.len() <= 1 {
    //     return Err(io::Error::new(io::ErrorKind::InvalidInput, "No arguments provided"));
    // }
    // let file_str = &args[1];

    // Hard code input
    let file_str = "input.txt";

    // Initialize constants
    let file = File::open(file_str)?;
    let reader = BufReader::new(file);
    let left: char = 'L';
    let right: char = 'R';
    // Mutable variables
    let mut dial: i16 = 50;
    let mut password1: i16 = 0;
    let mut password2: i16 = 0;
    // Loop over lines until there are no more
    for line in reader.lines() {
        let line = line?;
        // Get the dial direction (L or R)
        let direction: char = line.chars().nth(0).unwrap();
        // Get the magnitude
        let magnitude_str = &line[1..];
        let magnitude: i16 = magnitude_str.parse().expect("Unexpected integer read in input.");
        // Calculate the new dial location
        if direction == left {
            password2 += magnitude / 100;
            if dial != 0 && magnitude % 100 >= dial {
                password2 += 1;
            }
            dial = (dial - magnitude).rem_euclid(100);
        }
        else if direction == right  {
            dial += magnitude;
            password2 += dial / 100;
            dial = dial.rem_euclid(100);
        }
        // Add the number of rotations (times passed 0) to the password
        // Check for 0 match
        if dial == 0 {
            password1 = password1 + 1;
        }
    }
    // Stop timer
    let elapsed = start.elapsed();
    // Print Results
    println!("Execution time (seconds): {:?}", elapsed.as_secs_f32());
    println!("Password 1 is: {}", password1);
    println!("Password 2 is: {}", password2);

    Ok(())
}
