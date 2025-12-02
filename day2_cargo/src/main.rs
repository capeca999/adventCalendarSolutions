use regex::Regex;
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
    let mut sum_pairs = 0;
    let text_file_segmented = segment_text_file(clean_regex_file(file).unwrap());

    for line in text_file_segmented {
        //   println!("{}", line);
        let pulled_apart = pull_appart_pairs(&*line);
        sum_pairs += catch_pairs(pulled_apart.0, pulled_apart.1);
    }

    return Ok(sum_pairs);
}

fn clean_regex_file(file: File) -> Result<String, Error> {
    let reader = BufReader::new(file);
    let line = reader.lines().next().unwrap()?;
    return Ok(line.trim().to_string());
}

fn segment_text_file(line: String) -> Vec<String> {
    let re = Regex::new(r"\d+-\d+").unwrap();

    let matches: Vec<&str> = re.find_iter(line.as_str()).map(|m| m.as_str()).collect();

    let mut segments: Vec<String> = Vec::new();

    for m in matches {
        segments.push(m.to_string());
    }

    return segments;
}

fn pull_appart_pairs(pair: &str) -> (&str, &str) {
    let parts: Vec<&str> = pair.split('-').collect();
    (parts[0], parts[1])
}

fn catch_pairs(first: &str, second: &str) -> i64 {
    let mut ammount_of_pairs = 0;
    let mut first_growing = first.parse::<i64>().unwrap();

    while first_growing != second.parse::<i64>().unwrap() {
        let first_growing_str = first_growing.to_string();

        if first_growing_str.len() % 2 == 0 {
            if &first_growing_str[0..first_growing_str.len() / 2]
                == &first_growing_str[first_growing_str.len() / 2..first_growing_str.len()]
            {
                ammount_of_pairs += first_growing;
                //   println!("Pair found: {}/{}", first_growing_str, second);
            }
        }

        first_growing += 1;
    }

    ammount_of_pairs
}
