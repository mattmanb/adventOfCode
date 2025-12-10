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
        let line = line?;
        let digits: Vec<u32> = line
            .chars()
            .map(|c| c.to_digit(10).expect("Failed to convert char to digit"))
            .collect();

        let mut lgst = 0;
        for ld in 0..digits.len() - 1 {
            for rd in ld+1..digits.len() {
                let num = digits[ld] * 10 + digits[rd];
                if num > lgst {
                    lgst = num;
                }
            }
        }
        sum += lgst;

        // part 2
        let mut r_cutoff = digits.len() - 12;
        let mut lgst12: Vec<u32> = digits[r_cutoff..].to_vec();
        let mut l_cutoff: usize = 0;
        for i in 0..12 {
            let ind_of_largest = digits[l_cutoff..r_cutoff]
                .iter()
                .rev()
                .enumerate()
                .max_by_key(|&(_, &val)| val)
                .map(|(rev_index, _)| {
                    l_cutoff + (r_cutoff - l_cutoff) - 1 - rev_index
                });
            match ind_of_largest {
                Some(ind) => {
                    if digits[ind] > lgst12[i] {
                        lgst12[i] = digits[ind];
                        l_cutoff = ind + 1;
                        r_cutoff += 1;
                    } else {
                        break;
                    }
                },
                None => break,
            }
        }
        // Concatenate the lgst12 vector of numbers into an int and add it to the sum
        sum2 += lgst12.iter().fold(0u128, |acc, &elem| acc * 10 + elem as u128);
    }

    // Stop the timer
    let elapsed = start.elapsed();
    // Print Results
    println!("Execution time (seconds): {:?}", elapsed.as_secs_f32());
    println!("Sum: {}", sum);
    println!("Sum2: {}", sum2);

    Ok(())
}
