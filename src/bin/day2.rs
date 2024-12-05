use std::fs::read_to_string;

type Data = Vec<Vec<i32>>;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn parse(input: &str) -> Data {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|n| n.parse().ok())
                .collect()
        })
        .collect()
}

fn is_valid(row: &[i32]) -> bool {
    let increasing = row.windows(2).all(|w| w[1] > w[0] && (w[1] - w[0]).abs() <= 3);
    let decreasing = row.windows(2).all(|w| w[1] < w[0] && (w[1] - w[0]).abs() <= 3);
    return increasing || decreasing
}

fn is_valid_2(row: &[i32]) -> bool {
    (0..row.len()).any(|i| {
        // If we remove each index in the vector, check if we are valid
        let sequence: Vec<i32> = row[..i].iter().chain(row[i+1..].iter()).copied().collect();
        is_valid(&sequence)
    })
}

fn part1(data: &Data) -> Result<i32> {
    let result: usize = data.iter().filter(|row| is_valid(row)).count();
    Ok(result.try_into().unwrap())
}

fn part2(data: &Data) -> Result<i32> {
    let result: usize = data.iter().filter(|row| is_valid_2(row)).count();
    Ok(result.try_into().unwrap())
}

fn main() -> Result<()> {
    let data = parse(&read_to_string("input/day2.txt")?);
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

    const EXAMPLE: &str = include_str!("../../input/day2_test.txt");

    #[test]
    fn test_day2_part1() {
        let data = parse(EXAMPLE);
        assert_eq!(part1(&data).unwrap(), 2);
    }

    #[test]
    fn test_day2_part2() {
        let data = parse(EXAMPLE);
        assert_eq!(part2(&data).unwrap(), 4);
    }
}
