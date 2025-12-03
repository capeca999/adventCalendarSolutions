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

fn resolve_puzzle(file: File) -> Result<i64, Error> {

    let mut jolt = 0;

    let reader = BufReader::new(file);

    println!("Reading file line-by-line:\n");

    for line_result in reader.lines() {
        let line = line_result?;
        jolt += max_k_digits(&line, 12).parse::<i64>().unwrap();

    }

    return Ok(jolt);




}

fn max_k_digits(line: &str, k: usize) -> String {
    let chars: Vec<char> = line.chars().collect();
    let n = chars.len();
    let mut result = String::new();
    let mut start = 0;

    for remaining in (0..k).rev() {
        let mut best_digit = '0';
        let mut best_index = start;

        let end_limit = n - remaining - 1;

        for i in start..=end_limit {
            let d = chars[i];
            if d > best_digit {
                best_digit = d;
                best_index = i;

                if d == '9' {
                    break;
                }
            }
        }

        result.push(best_digit);
        start = best_index + 1;
    }

    result
}





