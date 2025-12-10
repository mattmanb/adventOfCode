use std::fs;
use std::io;
use std::time::Instant;

fn main() -> io::Result<()> {
    let start_time = Instant::now();

    let input_file = "input.txt";
    let contents = fs::read_to_string(input_file)?;

    let mut start = 0;
    let mut end: usize;
    let mut eof = false;
    let mut sum: u128 = 0;
    let mut sum2: u128 = 0;

    loop {
        let end_option = contents[start..].find(',');
        if end_option == None {
            end = contents.len() - 1;
            eof = true;
        } else {
            end = end_option.unwrap() + start;
        }
        let mid = contents[start..].find('-').unwrap() + start;
        let lbound: u128 = contents[start..mid].parse().expect("Failed to parse left boundary");
        let rbound: u128 = contents[mid+1..end].parse().expect("Failed to parse right boundary");

        for n in lbound..rbound+1 {
            let string_int = n.to_string();
            // part 2
            let string_int_copy = string_int.clone() + &string_int;
            // if the number from the second char to the penultimate char of the string_int + string_int contains string_int, that means it repeats more than just twice
            if string_int_copy[1..(2*string_int.len()-1)].contains(&string_int) {
                sum2 += n;
            }
            if string_int.len() % 2 == 1 {
                continue;
            }
            let midpoint = string_int.len() / 2;
            let str1 = &string_int[0..midpoint];
            let str2 = &string_int[midpoint..];
            if str1 == str2 {
                sum += n;
            }
        }

        if eof {
            break;
        } else {
            start = end + 1;
        }
    }
    let elapsed = start_time.elapsed();

    println!("Execution time (seconds): {:?}", elapsed.as_secs_f32());
    println!("Part 1 Sum: {}", sum);
    println!("Part 2 Sum: {}", sum2);

    Ok(())
}
