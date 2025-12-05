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
    let mut ranges: Vec<String> = Vec::new();
    let mut fruits: Vec<String> = Vec::new();

    let reader = BufReader::new(file);

    for line_result in reader.lines() {
        let line = line_result?.trim().to_string();
        if line.is_empty() {
            continue;
        }

        if line.contains('-') {
            ranges.push(line);

        } else {
            fruits.push(line);
        }
    }

    let mut fruit_not_spoiled_count = 0;

    for fruit in fruits {
        if !is_fruit_spoiler(&ranges, &fruit) {
            fruit_not_spoiled_count += 1;
        }
    }

    Ok(fruit_not_spoiled_count)
}


fn pull_appart_pairs(pair: &str) -> (&str, &str) {
    let parts: Vec<&str> = pair.split('-').map(|x| x.trim()).collect();
    (parts[0], parts[1])
}

fn is_fruit_spoiler(ranges: &Vec<String>, fruit: &str) -> bool {
    let fruit_value: i64 = fruit.trim().parse::<i64>().unwrap();

    for range in ranges {
        let (start, end) = pull_appart_pairs(range);

        let start_value: i64 = start.parse().unwrap();
        let end_value: i64 = end.parse().unwrap();

        if fruit_value >= start_value && fruit_value <= end_value {
            println!("Fruit {} is spoiled by range {}-{}", fruit_value, start_value, end_value);
            return false;
        }
    }
    println!("Fruit {} is not spoiled", fruit_value);
    return true;
}
