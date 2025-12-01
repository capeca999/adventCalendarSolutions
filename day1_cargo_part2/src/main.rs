use std::fs::File;
use std::io::{BufRead, BufReader, Error};


const TURN_LEFT_KEY: char = 'L';
const TURN_RIGHT_KEY: char = 'R';
const BREAK_POINT_KEY: i32 = 0;

struct Status{
    times_passed_break_point: i32,
    newPos: i32,
}


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

fn resolve_puzzle(file: File) -> Result<i32, Error> {
print!("ESTAMOS POR EL 50");
    let mut actual_position = 50;
    let mut times_reached_break_point = 0;

    let reader = BufReader::new(file);

    println!("Reading file line-by-line:\n");

    for line_result in reader.lines() {
        let line = line_result?;
        let first_char = line.chars().next().unwrap();

        let status = if first_char == 'L' {
            turn_left(actual_position, line)
        } else {
            turn_right(actual_position, line)
        };

        times_reached_break_point += status.times_passed_break_point;
        actual_position = status.newPos;

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

fn calculate_distance_left(pos : i32, mov : i32) -> i32 {
    return pos - mov;
}

fn calculate_distance_right(pos : i32, mov : i32) -> i32 {
    return pos + mov;
}


fn calculate_times_passed_break_point(pos_inicial: i32, mov: i32, is_left: bool) -> Status {

    let mut pos = pos_inicial;
    let mut passed = 0;

    let direction = if is_left { -1 } else { 1 };

    for _ in 0..mov {
        pos += direction;

        if pos < 0 {
            pos = 99;
        } else if pos > 99 {
            pos = 0;
        }

        if pos == 0 {
            passed += 1;
        }
    }

    Status {
        times_passed_break_point: passed,
        newPos: pos,
    }
}



fn build_status(times_passed_break_point: i32, new_post: i32) -> Status{
    Status{
        times_passed_break_point: times_passed_break_point,
        newPos: new_post
    }
}

