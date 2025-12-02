use std::fs::File;
use std::io::{BufRead, BufReader, Error};

const TURN_LEFT_KEY: char = 'L';
const BREAK_POINT_KEY: i32 = 0;
const STARTING_POS: i32 = 50;

struct Status {
    times_passed_break_point: i32,
    new_pos: i32,
}

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
    let mut actual_position = STARTING_POS;
    let mut times_reached_break_point = 0;

    let reader = BufReader::new(file);

    println!("Reading file line-by-line:\n");

    for line_result in reader.lines() {
        let line = line_result?;
        let first_char = line.chars().next().unwrap();

        let status = if first_char == TURN_LEFT_KEY {
            turn_left(actual_position, line)
        } else {
            turn_right(actual_position, line)
        };

        times_reached_break_point += status.times_passed_break_point;
        actual_position = status.new_pos;
    }

    return Ok(times_reached_break_point);
}

fn turn_left(pos: i32, line: String) -> Status {
    let distance: i32 = line[1..].parse().unwrap();
    calculate_times_passed_break_point(pos, distance, true)
}

fn turn_right(pos: i32, line: String) -> Status {
    let distance: i32 = line[1..].parse().unwrap();
    calculate_times_passed_break_point(pos, distance, false)
}

fn calculate_times_passed_break_point(pos_inicial: i32, mov: i32, is_left: bool) -> Status {
    let mut pos = pos_inicial;
    let mut passed = 0;

    let direction = if is_left { -1 } else { 1 };

    for _ in 0..mov {
        pos += direction;

        if pos < BREAK_POINT_KEY {
            pos = 99;
        } else if pos > 99 {
            pos = BREAK_POINT_KEY;
        }

        if pos == BREAK_POINT_KEY {
            passed += 1;
        }
    }

    Status {
        times_passed_break_point: passed,
        new_pos: pos,
    }
}
