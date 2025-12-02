use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
fn main() {
    let file_path = "src/puzzleInput.txt";
    let file = File::open(file_path);
    match file {
        Ok(f) => match resolve_puzzle(f) {
            Ok(result) => println!("Puzzle solved: {}", result),
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

    Ok(sum_pairs)
}

fn clean_regex_file(file: File) -> Result<String, Error> {
    let reader = BufReader::new(file);
    let line = reader.lines().next().unwrap()?;

    Ok(line.trim().to_string())
}

fn segment_text_file(line: String) -> Vec<String> {
    let re = Regex::new(r"\d+-\d+").unwrap();

    let matches: Vec<&str> = re.find_iter(line.as_str()).map(|m| m.as_str()).collect();

    let mut segments: Vec<String> = Vec::new();

    for m in matches {
        segments.push(m.to_string());
    }

    segments
}

fn pull_appart_pairs(pair: &str) -> (&str, &str) {
    let parts: Vec<&str> = pair.split('-').collect();
    (parts[0], parts[1])
}

fn catch_pairs(first: &str, second: &str) -> i64 {
    let start = first.parse::<i64>().unwrap();
    let end = second.parse::<i64>().unwrap();

    let mut sum = 0;

    for n in start..=end {
        if is_repeated_pattern(n) {
            sum += n;
        }
    }

    sum
}

fn is_repeated_pattern(n: i64) -> bool {
    let s = n.to_string();
    let len = s.len();

    for block in 1..=(len / 2) {
        if len % block != 0 {
            continue;
        }

        let repetitions = len / block;

        if repetitions < 2 {
            continue;
        }

        let part = &s[0..block];

        if part.repeat(repetitions) == s {
            return true;
        }
    }

    false
}
