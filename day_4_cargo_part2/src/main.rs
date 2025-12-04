use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::thread;
use std::thread::Thread;
use std::time::Duration;

const AT_SIGN: char = '@';

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
    let reader = BufReader::new(file);
    let mut has_something_changed:bool;
    let mut matrix: Vec<Vec<char>> = Vec::new();
    for line_result in reader.lines() {
        let line = line_result?;
        let chars: Vec<char> = line.chars().collect();
        matrix.push(chars);
    }

    let mut times_reached_break_point = 0;

    let height = matrix.len();
    let width = matrix[0].len();
    let mut matrix_mirror = matrix.clone();

    loop{
        has_something_changed = false;
        matrix = matrix_mirror.clone();
        for row in 0..height {
            for character in 0..width {
                if matrix[row][character] == AT_SIGN {

                    if matrix[row][character] == AT_SIGN && check_availability(&mut matrix, height, width, row, character) < 4 {
                        matrix_mirror[row][character] = '.';
                        times_reached_break_point += 1;
                        has_something_changed = true;
                    }
                }
            }
        }
        if has_something_changed == false {
            break;
        }

        println!("{}", matrix_mirror.iter().map(|r| r.iter().collect::<String>()).collect::<Vec<String>>().join("\n"));
        thread::sleep(Duration::from_millis(125));

    }
    Ok(times_reached_break_point)

}

fn check_availability(matriz: &mut Vec<Vec<char>>, height: usize, width: usize, fila: usize, character: usize) -> i32 {
    let mut count = 0;
    if fila + 1 < height && matriz[fila + 1][character] == AT_SIGN { count += 1; }
    if fila >= 1 && matriz[fila - 1][character] == AT_SIGN { count += 1; }
    if character + 1 < width && matriz[fila][character + 1] == AT_SIGN { count += 1; }
    if character >= 1 && matriz[fila][character - 1] == AT_SIGN { count += 1; }

    if fila + 1 < height && character + 1 < width && matriz[fila + 1][character + 1] == AT_SIGN { count += 1; }
    if fila + 1 < height && character >= 1 && matriz[fila + 1][character - 1] == AT_SIGN { count += 1; }
    if fila >= 1 && character + 1 < width && matriz[fila - 1][character + 1] == AT_SIGN { count += 1; }
    if fila >= 1 && character >= 1 && matriz[fila - 1][character - 1] == AT_SIGN { count += 1; }
    count
}




