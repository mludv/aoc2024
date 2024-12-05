use std::fs::read_to_string;

use itertools::Itertools;

type Data = Vec<Vec<char>>;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn parse(input: &str) -> Data {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[allow(dead_code)]
fn print_grid(data: &Vec<Vec<char>>, highlights: Option<&[(usize, usize)]>) {
    let highlight_set: std::collections::HashSet<_> = highlights
        .map(|h| h.iter().cloned().collect())
        .unwrap_or_default();
    for (i, row) in data.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if highlight_set.contains(&(i, j)) {
                print!("\x1b[91m{}\x1b[0m", cell); // Red highlight
            } else {
                print!("{}", cell);
            }
        }
        println!();
    }
}

fn bounded_grid_index(pos: usize, delta: i32, upper_bound: usize) -> Option<usize> {
    // Return the new grid index if it is within the grid, otherwise None
    let new_pos = (pos as i32) + delta;
    if new_pos >= 0 && new_pos < upper_bound as i32 {
        Some(new_pos as usize)
    } else {
        None
    }
}

fn part1(data: &Data) -> Result<usize> {
    let rows = data.len();
    let cols = data[0].len();
    const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];
    let dirs: [(i32, i32); 8] = [
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];

    let result: usize = (0..rows)
        .cartesian_product(0..cols)
        .filter(|&(i, j)| data[i][j] == 'X')
        .map(|(i, j)| {
            dirs.iter()
                .filter(|&&(dx, dy)| {
                    (0..4).all(|k: usize| {
                        let ni = bounded_grid_index(i, dx * (k as i32), rows);
                        let nj = bounded_grid_index(j, dy * (k as i32), cols);
                        if let (Some(ni), Some(nj)) = (ni, nj) {
                            return data[ni][nj] == XMAS[k];
                        } else {
                            return false;
                        }
                    })
                })
                .count()
        })
        .sum();
    Ok(result)
}

fn part2(data: &Data) -> Result<usize> {
    let rows = data.len();
    let cols = data[0].len();
    // The two diagonals
    let dirs: [(i32, i32); 2] = [(1, 1), (1, -1)];

    let result: usize = (0..rows)
        .cartesian_product(0..cols)
        .filter(|&(i, j)| data[i][j] == 'A')
        .filter(|&(i, j)| {
            // We need to match both diagonals in either direction
            dirs.iter().all(|&(dx, dy)| {
                // Check if either direction writes MAS
                [1, -1].iter().any(|&k| {
                    let ai = bounded_grid_index(i, dx * k, rows);
                    let aj = bounded_grid_index(j, dy * k, cols);
                    let bi = bounded_grid_index(i, -dx * k, rows);
                    let bj = bounded_grid_index(j, -dy * k, cols);
                    if let (Some(ai), Some(aj), Some(bi), Some(bj)) = (ai, aj, bi, bj) {
                        if data[ai][aj] == 'M' && data[bi][bj] == 'S' {
                            // println!("TRUE");
                            return true;
                        }
                    }
                    return false;
                })
            })
        })
        .count();
    Ok(result)
}

fn main() -> Result<()> {
    let data = parse(&read_to_string("input/day4.txt")?);
    // Raw input as string
    let answer1 = part1(&data)?;
    println!("Part 1: {}", answer1);

    let answer2 = part2(&data)?;
    println!("Part 2: {}", answer2);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
"#;

    #[test]
    fn test_day2_part1() {
        let data = parse(EXAMPLE);
        assert_eq!(part1(&data).unwrap(), 18);
    }

    #[test]
    fn test_day2_part2() {
        let data = parse(EXAMPLE);
        assert_eq!(part2(&data).unwrap(), 9);
    }
}
