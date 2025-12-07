use std::collections::HashMap;

fn count_timelines(input: &str) -> u64 {
    let grid: Vec<Vec<char>> = input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.chars().collect())
        .collect();

    let rows = grid.len();
    let cols = grid[0].len();

    // localizar S
    let mut start = None;
    for (r, row) in grid.iter().enumerate() {
        if let Some(c) = row.iter().position(|&ch| ch == 'S') {
            start = Some((r, c));
            break;
        }
    }
    let (sr, sc) = start.expect("S no encontrada");

    let mut memo = HashMap::<(usize, usize), u64>::new();

    fn dfs(
        r: usize,
        c: usize,
        grid: &Vec<Vec<char>>,
        memo: &mut HashMap<(usize, usize), u64>,
    ) -> u64 {
        let rows = grid.len();
        let cols = grid[0].len();

        if r + 1 >= rows {
            // sale por abajo → timeline completo
            return 1;
        }

        if let Some(&v) = memo.get(&(r, c)) {
            return v;
        }

        let cell = grid[r + 1][c];
        let result = match cell {
            '.' | 'S' => {
                dfs(r + 1, c, grid, memo)
            }
            '^' => {
                let mut total = 0;

                if c > 0 {
                    total += dfs(r + 1, c - 1, grid, memo);
                }
                if c + 1 < cols {
                    total += dfs(r + 1, c + 1, grid, memo);
                }

                total
            }
            _ => unreachable!(),
        };

        memo.insert((r, c), result);
        result
    }

    dfs(sr, sc, &grid, &mut memo)
}

fn main() {
    let input = include_str!("input.txt");
    let result = count_timelines(input);
    println!("Total timelines: {}", result);
}
