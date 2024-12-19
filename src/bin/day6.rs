use std::fs::read_to_string;

use itertools::Itertools;

type Data = Vec<Vec<char>>;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn parse(input: &str) -> Data {
    input.lines().map(|row| row.chars().collect()).collect()
}

fn part1(data: &Data) -> Result<usize> {
    let (x, y) = data.iter().enumerate()
        .find_map(|(i, row)| 
            row.iter().enumerate().find(|(_, &ch)| ch == '^').map(|(j, _)| (i, j))
        ).expect("Has an initial position");

    loop {
        data[x][y]
    }
    println!("Initial: {:?}", initial);
    Ok(0)
}

fn part2(data: &Data) -> Result<usize> {
    Ok(0)
}

fn main() -> Result<()> {
    let data = parse(&read_to_string("input/day5.txt")?);
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

    const EXAMPLE: &str = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
"#;

    #[test]
    fn test_day6_part1() {
        let data = parse(EXAMPLE);
        assert_eq!(part1(&data).unwrap(), 143);
    }

    #[test]
    fn test_day6_part2() {
        let data = parse(EXAMPLE);
        assert_eq!(part2(&data).unwrap(), 123);
    }
}
