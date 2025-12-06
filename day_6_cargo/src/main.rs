use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;
use num_bigint::BigInt;
use num_traits::Zero;

fn main() {
    let file_path = "src/puzzleInput.txt";
    let file = File::open(file_path).unwrap();

    let final_total = solve_puzzle(file);

    println!("✨ Puzzle solved: {} ✨", final_total);
}

fn solve_puzzle(file: File) -> BigInt {
    let reader = BufReader::new(file);

    // Leer todas las líneas
    let raw_lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let row_count = raw_lines.len();
    let col_count = raw_lines.iter().map(|l| l.len()).max().unwrap();

    // Normalizar todas las líneas para que tengan el mismo ancho
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

    // Regex para extraer números y operadores verticales
    let regex_number = Regex::new(r"\d+").unwrap();
    let regex_operator = Regex::new(r"[+*]").unwrap();

    let mut grand_total = BigInt::zero();
    let mut current_col = 0;

    // Recorremos las columnas de izquierda a derecha
    while current_col < col_count {

        // 1. Saltar columnas completamente vacías
        let is_empty_column = (0..row_count).all(|r| normalized_grid[r][current_col] == ' ');

        if is_empty_column {
            current_col += 1;
            continue;
        }

        // 2. Detectar bloque continuo de columnas perteneciente a un solo problema
        let block_start_col = current_col;

        while current_col < col_count
            && (0..row_count).any(|r| normalized_grid[r][current_col] != ' ')
        {
            current_col += 1;
        }

        let block_end_col = current_col; // No incluido

        // 3. Construir las líneas verticales del bloque de columnas
        let mut vertical_lines: Vec<String> = vec![String::new(); row_count];

        for row in 0..row_count {
            for col in block_start_col..block_end_col {
                vertical_lines[row].push(normalized_grid[row][col]);
            }
        }

        // 4. Extraer operador de la última fila del bloque
        let operator_line = &vertical_lines[row_count - 1];
        let operator_match = regex_operator.find(operator_line).unwrap();
        let operator = operator_match.as_str().chars().next().unwrap();

        // 5. Extraer números verticales del resto del bloque
        let mut problem_numbers: Vec<BigInt> = Vec::new();

        for row in 0..row_count - 1 {
            let line_content = &vertical_lines[row];

            if let Some(number_match) = regex_number.find(line_content) {
                let value = number_match.as_str().parse::<BigInt>().unwrap();
                problem_numbers.push(value);
            }
        }

        // 6. Evaluar el problema (suma o multiplicación)
        let mut problem_result = problem_numbers[0].clone();

        for value in &problem_numbers[1..] {
            if operator == '+' {
                problem_result += value;
            } else {
                problem_result *= value;
            }
        }

        // 7. Añadir resultado al total general
        grand_total += problem_result;
    }

    grand_total
}
