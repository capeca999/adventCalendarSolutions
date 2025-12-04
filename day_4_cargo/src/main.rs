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
    let reader = BufReader::new(file);

    let mut matriz: Vec<Vec<char>> = Vec::new();

    for line_result in reader.lines() {
        let line = line_result?;
        let chars: Vec<char> = line.chars().collect();
        matriz.push(chars);
    }

    let mut times_reached_break_point = 0;

    let height = matriz.len();
    let width = matriz[0].len();

    for fila in 0..height {
        for caracter in 0..width {

            if matriz[fila][caracter] == '@' {

                let mut count = 0;

                if fila + 1 < height && matriz[fila + 1][caracter] == '@' { count += 1; }
                if fila >= 1               && matriz[fila - 1][caracter] == '@' { count += 1; }
                if caracter + 1 < width && matriz[fila][caracter + 1] == '@' { count += 1; }
                if caracter >= 1               && matriz[fila][caracter - 1] == '@' { count += 1; }

                if fila + 1 < height && caracter + 1 < width && matriz[fila + 1][caracter + 1] == '@' { count += 1; }
                if fila + 1 < height && caracter >= 1    && matriz[fila + 1][caracter - 1] == '@' { count += 1; }
                if fila >= 1    && caracter + 1 < width && matriz[fila - 1][caracter + 1] == '@' { count += 1; }
                if fila >= 1    && caracter >= 1    && matriz[fila - 1][caracter - 1] == '@' { count += 1; }

                if count < 4 {
                    times_reached_break_point += 1;
                }
            }
        }
    }

    Ok(times_reached_break_point)
}




