use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let start = Instant::now();

    let file_path = "src/puzzleInput.txt";
    let file = File::open(file_path);
    match file {
        Ok(f) => match resolve_puzzle(f) {
            Ok(result) => println!("\x1b[1;93m✨ Puzzle solved: {} ✨\x1b[0m IN {}", result, start.elapsed().as_secs_f32()),
            Err(e) => eprintln!("Error solving puzzle: {}", e),
        },
        Err(e) => eprintln!("Error opening file {}: {}", file_path, e),
    }
}

fn resolve_puzzle(file: File) -> Result<i64, Error> {

    let mut jolt = 0;

    let reader = BufReader::new(file);

    println!("Reading file line-by-line:\n");

    for line_result in reader.lines() {
        let line = line_result?;
        jolt += max_k_digits_animated(&line, 12).parse::<i64>().unwrap();

    }

    return Ok(jolt);




}


fn max_k_digits_animated(line: &str, k: usize) -> String {
    let chars: Vec<char> = line.chars().collect();
    let n = chars.len();
    let mut result = String::new();
    let mut start = 0;

    let spinner = ["|", "/", "-", "\\"];

    for (step, remaining) in (0..k).rev().enumerate() {
        let mut best_digit = '0';
        let mut best_index = start;

        let end_limit = n - remaining - 1;

        for (i, ch) in chars.iter().enumerate() {
            if i < start || i > end_limit {
                continue;
            }

            // 🔄 animación: limpiar pantalla
            print!("\x1b[2J\x1b[H");

            // 🌀 spinner animado
            let spin = spinner[(i % spinner.len())];

            println!("\x1b[1;96m=== Construyendo dígito {}/{} ===\x1b[0m", step + 1, k);
            println!("{}  \x1b[93mAnalizando índice {} → dígito {}\x1b[0m\n", spin, i, ch);

            // Mostrar la línea original con colores
            println!("\x1b[90mLínea completa:\x1b[0m\n");

            for (j, c) in chars.iter().enumerate() {
                if j == i {
                    print!("\x1b[93m{}\x1b[0m", c); // amarillo → actual evaluado
                } else if j == best_index {
                    print!("\x1b[92m{}\x1b[0m", c); // verde → el mejor encontrado hasta ahora
                } else if j < start || j > end_limit {
                    print!("\x1b[90m{}\x1b[0m", c); // gris → fuera del rango posible
                } else {
                    print!("{}", c); // blanco normal
                }
            }

            println!("\n");

            // Mostrar el mejor dígito por ahora
            println!(
                "🟢 Mejor dígito hasta ahora: \x1b[1;92m{}\x1b[0m en posición {}",
                best_digit, best_index
            );

            // Mostrar el resultado parcial
            println!(
                "\nResultado parcial: \x1b[1;94m{}\x1b[0m",
                result
            );

            thread::sleep(Duration::from_millis(45));

            // Ahora la lógica real:
            if i >= start && i <= end_limit && *ch > best_digit {
                best_digit = *ch;
                best_index = i;

                if best_digit == '9' {
                    break;
                }
            }
        }

        // Añadir al resultado
        result.push(best_digit);
        start = best_index + 1;
    }

    result
}






