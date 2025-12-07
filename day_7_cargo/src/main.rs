use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");
    let result = count_splits(input);
    println!("Número de divisiones del haz: {}", result);
}

fn count_splits(input: &str) -> usize {
    // 1. Parseamos el mapa en una matriz de chars
    let grid: Vec<Vec<char>> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.chars().collect())
        .collect();

    let rows = grid.len();
    let cols = grid[0].len();

    // 2. Buscamos la S (punto de entrada del haz)
    let mut start = None;
    for (r, row) in grid.iter().enumerate() {
        if let Some(c) = row.iter().position(|&ch| ch == 'S') {
            start = Some((r, c));
            break;
        }
    }

    let (start_r, start_c) = start.expect("No se ha encontrado 'S' en el mapa");

    // 3. Conjunto de haces activos: cada haz es una posición (fila, columna)
    let mut active: HashSet<(usize, usize)> = HashSet::new();
    active.insert((start_r, start_c));

    let mut splits = 0;

    // 4. Simulación: mientras haya algún haz activo
    while !active.is_empty() {
        let mut next_active: HashSet<(usize, usize)> = HashSet::new();

        for &(r, c) in &active {
            let nr = r + 1; // el haz siempre baja

            // Si nos salimos por abajo, este haz desaparece
            if nr >= rows {
                continue;
            }

            match grid[nr][c] {
                '^' => {
                    // El haz se detiene aquí y se divide
                    splits += 1;

                    // Haz nuevo a la izquierda
                    if c > 0 {
                        next_active.insert((nr, c - 1));
                    }
                    // Haz nuevo a la derecha
                    if c + 1 < cols {
                        next_active.insert((nr, c + 1));
                    }
                }
                _ => {
                    // Espacio vacío (o cualquier cosa que no sea splitter): el haz sigue bajando
                    next_active.insert((nr, c));
                }
            }
        }

        // Unimos haces que hayan llegado a la misma casilla (HashSet ya hace el merge)
        active = next_active;
    }

    splits
}
