use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;
use num_bigint::BigInt;
use num_traits::{Zero, One};

fn main() {
    let file_path = "src/puzzleInput.txt";
    let file = File::open(file_path).unwrap();

    let total_part2 = solve_puzzle_part2(file);

    println!("✨ Puzzle solved (Part 2): {} ✨", total_part2);
}

fn solve_puzzle_part2(file: File) -> BigInt {
    let reader = BufReader::new(file);
    let raw_lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let row_count = raw_lines.len();
    let col_count = raw_lines.iter().map(|l| l.len()).max().unwrap();

    // Normalizar filas para que todas tengan el mismo ancho
    let normalized_grid: Vec<Vec<char>> = raw_lines
        .into_iter()
        .map(|line| {
            let mut chars: Vec<char> = line.chars().collect();
            while chars.len() < col_count {
                chars.push(' ');
            }
            chars
        })
        .collect();

    let regex_digit = Regex::new(r"\d").unwrap();
    let regex_operator = Regex::new(r"[+*]").unwrap();

    let mut grand_total = BigInt::zero();

    let mut current_col: i32 = (col_count as i32) - 1;

    while current_col >= 0 {
        let col = current_col as usize;

        // Saltar columnas completamente vacías
        let is_empty_column = (0..row_count).all(|r| normalized_grid[r][col] == ' ');
        if is_empty_column {
            current_col -= 1;
            continue;
        }

        // Detectar bloque de problema hacia la izquierda
        let block_end = col + 1; // No incluido
        let mut block_start = col;

        while block_start > 0 {
            let prev_col = block_start - 1;
            let prev_is_empty = (0..row_count).all(|r| normalized_grid[r][prev_col] == ' ');
            if prev_is_empty { break }
            block_start -= 1;
        }

        let mut problem_numbers: Vec<BigInt> = Vec::new();

        for column in (block_start..block_end).rev() { // derecha → izquierda
            let mut digits = String::new();

            for row in 0..(row_count - 1) { // de arriba → abajo
                let ch = normalized_grid[row][column];

                if regex_digit.is_match(&ch.to_string()) {
                    digits.push(ch);
                }
            }

            if !digits.is_empty() {
                problem_numbers.push(digits.parse::<BigInt>().unwrap());
            }
        }


        // Operador está en la última fila del bloque
        let mut operator = '+';
        for c in (block_start..block_end).rev() {
            let ch = normalized_grid[row_count - 1][c];
            if regex_operator.is_match(&ch.to_string()) {
                operator = ch;
                break;
            }
        }

        // Evaluar el problema
        let mut problem_result = problem_numbers[0].clone();
        for n in &problem_numbers[1..] {
            if operator == '+' {
                problem_result += n;
            } else {
                problem_result *= n;
            }
        }

        grand_total += problem_result;

        current_col = block_start as i32 - 1;
    }

    grand_total
}
