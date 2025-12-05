use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn main() {
    let file_path = "src/puzzleInput.txt";
    let file = File::open(file_path);
    match file {
        Ok(f) => match solve_part2(f) {
            Ok(result) => println!("\x1b[1;93m✨ Puzzle solved: {} ✨\x1b[0m", result),
            Err(e) => eprintln!("Error solving puzzle: {}", e),
        },
        Err(e) => eprintln!("Error opening file {}: {}", file_path, e),
    }
}

fn solve_part2(file: File) -> Result<u64, Error> {
    let mut ranges: Vec<(u64, u64)> = Vec::new();

    for line in BufReader::new(file).lines() {
        let line = line?.trim().to_string();
        if line.is_empty() { continue; }

        if line.contains('-') {
            let (a, b) = parse_range(&line);
            ranges.push((a, b));
        }
    }

    // 1. Ordenar rangos por inicio
    ranges.sort_by_key(|r| r.0);

    // 2. Fusionar rangos
    let merged = merge_ranges(&ranges);

    // 3. Contar todos los IDs fresh
    let total: u64 = merged.iter().map(|(s, e)| (e - s + 1)).sum();

    Ok(total)
}

fn parse_range(s: &str) -> (u64, u64) {
    let parts: Vec<&str> = s.split('-').collect();
    let a = parts[0].parse::<u64>().unwrap();
    let b = parts[1].parse::<u64>().unwrap();
    if a <= b { (a, b) } else { (b, a) }
}

fn merge_ranges(ranges: &Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    let mut merged: Vec<(u64, u64)> = Vec::new();

    for &(start, end) in ranges {
        if let Some(last) = merged.last_mut() {
            // si solapan o tocan, fusionar
            if start <= last.1 + 1 {
                last.1 = last.1.max(end);
                continue;
            }
        }
        // si no, es un rango nuevo
        merged.push((start, end));
    }

    merged
}