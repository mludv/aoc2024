use regex::Regex;
use std::fs::read_to_string;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn part1(input: &str) -> Result<i32> {
    let mul = Regex::new(r"mul\((\d+),(\d+)\)")?;
    let total = mul
        .captures_iter(input)
        .map(|caps| {
            let a: i32 = caps[1].parse().unwrap();
            let b: i32 = caps[2].parse().unwrap();
            a * b
        })
        .sum();
    Ok(total)
}

fn part2(input: &str) -> Result<i32> {
    let mul = Regex::new(r"mul\((\d+),(\d+)\)")?;
    let total = input
        .split("do()")
        .map(|text| {
            // only match the first one for each slice
            println!("===========");
            println!("{}", text);
            let active_part = text.split("don't()").next().unwrap();
            println!("{}", active_part);
            if let Some(caps) = mul.captures(active_part) {
                let a: i32 = caps[1].parse().unwrap();
                let b: i32 = caps[2].parse().unwrap();
                println!("{:?}", (a, b));
                return a * b;
            }
            return 0;
        })
        .sum();

    Ok(total)
}

fn main() -> Result<()> {
    let data = read_to_string("input/day3.txt")?;
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

    const EXAMPLE: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const EXAMPLE2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_day2_part1() {
        let data = EXAMPLE;
        assert_eq!(part1(&data).unwrap(), 161);
    }
    #[test]
    fn test_day2_part2() {
        let data = EXAMPLE2;
        assert_eq!(part2(&data).unwrap(), 48);
    }
}
