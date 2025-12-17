use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::time::Instant;
use std::cmp;

fn main() -> io::Result<()> {
    let start_time = Instant::now();

    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut forklift: Vec<Vec<char>> = vec![];

    for line in reader.lines() {
        let char_vec: Vec<char> = line.unwrap().chars().collect();
        forklift.push(char_vec.clone());
    }

    let mut acc_paper_count = 0;

    let rc: i32 = forklift.len() as i32;
    let cc: i32 = forklift[0].len() as i32;

    loop {

        let mut acc_paper_inds: Vec<(usize, usize)> = vec![];

        for (row, forklift_row) in forklift.iter().enumerate() {
            for (col, occ) in forklift_row.iter().enumerate() {
                if *occ == '.' {
                    continue;
                }
                let ri: i32 = row as i32;
                let ci: i32 = col as i32;
                // { // Adjacent locations
                //     [ii, ji], [ii+1, ji+1], ..., [ij, jj]
                // }
                let ii: i32 = cmp::max(ri - 1, 0);
                let ij: i32 = cmp::min(ri + 1, cc-1);
                let ji: i32 = cmp::max(ci - 1, 0);
                let jj: i32 = cmp::min(ci + 1, rc-1);

                let mut adj_paper_count = 0;

                for i in ii..ij+1 {
                    for j in ji..jj+1 {
                        if forklift[i as usize][j as usize] == '@' {
                            adj_paper_count += 1;
                        }
                    }
                }
                if adj_paper_count <= 4 {
                    acc_paper_count += 1;
                    // part 2 (save indices of accessible paper)
                    acc_paper_inds.push((row, col));
                }
            }
        }
        // part 2
        if acc_paper_inds.len() == 0 {
            break;
        } else {
            for (r, c) in acc_paper_inds {
                forklift[r][c] = '.';
            }
        }
        iteration += 1;

    }

    let elapsed = start_time.elapsed();
    // Print Results
    println!("Execution time (seconds): {:?}", elapsed.as_secs_f32());
    println!("Accessible rolls of paper: {}", acc_paper_count);


    Ok(())
    
}

