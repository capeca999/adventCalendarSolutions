use std::fs::File;
use std::io::{BufRead, BufReader, Error};



fn main() {
    let file_path = "src/puzzleInput.txt";
    let file = File::open(file_path);
    match file {
        Ok(f) => match resolve_puzzle(f) {
            Ok(result) => println!("\x1b[1;93m✨ Puzzle solved: {} ✨\x1b[0m", result),
            Err(e) => eprintln!("Error solving puzzle: {}", e),
        },
        Err(e) => eprintln!("Error opening file {}: {}", file_path, e),
    }
}

fn resolve_puzzle(file: File) -> Result<i32, Error> {

    let mut jolt = 0;

    let reader = BufReader::new(file);

    println!("Reading file line-by-line:\n");

    for line_result in reader.lines() {
        let line = line_result?;
       jolt += get_max_jolt(&line);

    }

    return Ok(jolt);




}

fn get_max_jolt(line: &str) -> i32 {
    let chars: Vec<i32> = line.chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect();

    let mut max_val = 0;

    for i in 0..chars.len() {
        let a = chars[i];

        let mut max_b = -1;

        for j in i+1..chars.len() {
            let b = chars[j];
            if b > max_b {
                max_b = b;
            }
        }

        if max_b >= 0 {
            let val = a * 10 + max_b;
            if val > max_val {
                max_val = val;
            }
        }
    }

    max_val
}




