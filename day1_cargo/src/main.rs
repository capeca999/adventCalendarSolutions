use std::fs::File;
use std::io::{BufRead, BufReader, Error};


const TURN_LEFT_KEY: char = 'L';
const TURN_RIGHT_KEY: char = 'R';
const BREAK_POINT_KEY: i32 = 0;

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

    let mut actual_position = 50;
    let mut times_reached_break_point = 0;

    let reader = BufReader::new(file);

    println!("Reading file line-by-line:\n");

    for line_result in reader.lines() {
        let line = line_result?;
        let first_char = line.chars().next().unwrap();

        if first_char == TURN_LEFT_KEY {
            // Turn left
            actual_position = turn_left(actual_position, line);
            if actual_position == BREAK_POINT_KEY {
                times_reached_break_point = times_reached_break_point + 1;
            }
        } else if first_char == TURN_RIGHT_KEY {
            // Turn right
            actual_position = turn_right(actual_position, line);
            if actual_position == BREAK_POINT_KEY {
                times_reached_break_point = times_reached_break_point + 1;
            }
        }
    }

    return Ok(times_reached_break_point);




}

fn turn_left(pos: i32, line: String) -> i32 {

    let distance: i32 = line[1..].parse().unwrap();
    return calculate_distance(pos, -distance);
}

fn turn_right(pos: i32, line: String) -> i32 {
    let distance: &str = &line[1..];
    return calculate_distance(pos, distance.parse::<i32>().unwrap());
}

fn calculate_distance(pos : i32, mov : i32) -> i32 {

    let mut new_pos = (pos + mov) % 100;

    if new_pos < 0 {
        new_pos += 100;
    }

    new_pos

}

