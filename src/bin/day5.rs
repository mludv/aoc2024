use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

use itertools::Itertools;

type Data = (Vec<(usize, usize)>, Vec<Vec<usize>>);
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn parse(input: &str) -> Data {
    let (rules, updates) = input.split_once("\n\n").unwrap();
    let rules: Vec<(usize, usize)> = rules
        .lines()
        .map(|row| {
            let (a, b) = row.split_once('|').unwrap();
            let a: usize = a.parse().unwrap();
            let b: usize = b.parse().unwrap();
            (a, b)
        })
        .collect();
    let updates: Vec<Vec<usize>> = updates
        .lines()
        .map(|row| row.split(',').map(|x| x.parse().unwrap()).collect())
        .collect();
    (rules, updates)
}

fn part1(data: &Data) -> Result<usize> {
    let (rules, updates) = data;
    // let before: HashMap<usize, HashSet<usize>> = rules.sort
    let before_rules: HashMap<usize, HashSet<usize>> = rules
        .iter()
        .copied()
        .into_group_map()
        .into_iter()
        .map(|(k, v)| {
            let set: HashSet<usize> = v.into_iter().collect();
            (k, set)
        })
        .collect();
    let total: usize = updates
        .into_iter()
        .map(|row| {
            println!("====================");
            println!("{:?}", row);
            let valid = (0..row.len()).all(|i| {
                let x = row[i];
                if let Some(rules) = before_rules.get(&x) {
                    let forward: bool = row[i + 1..].iter().all(|b| rules.contains(b));
                    let back: bool = row[..i].iter().all(|b| !rules.contains(b));
                    return forward && back;
                } else {
                    return true;
                }
            });
            if valid {
                println!("CORRECT: {:?}", row);
                row[row.len() / 2]
            } else {
                0
            }
        })
        .sum();
    Ok(total)
}

fn part2(data: &Data) -> Result<usize> {
    let (rules, updates) = data;
    // let before: HashMap<usize, HashSet<usize>> = rules.sort
    let before_rules: HashMap<usize, HashSet<usize>> = rules
        .iter()
        .copied()
        .into_group_map()
        .into_iter()
        .map(|(k, v)| {
            let set: HashSet<usize> = v.into_iter().collect();
            (k, set)
        })
        .collect();
    println!("{:?}", before_rules);
    let total: usize = updates
        .into_iter()
        .map(|row| {
            let valid = (0..row.len()).all(|i| {
                let x = row[i];
                if let Some(rules) = before_rules.get(&x) {
                    let compliant: bool = row[i + 1..].iter().all(|b| rules.contains(b));
                    let back: bool = row[..i].iter().all(|b| !rules.contains(b));
                    return compliant && back;
                } else {
                    return true;
                }
            });
            if valid {
                0
            } else {
                println!("====================");
                println!("SHOULD ORDER: {:?}", row);
                let sorted: Vec<usize> = row
                    .iter()
                    .copied()
                    .sorted_by(|a, b| {
                        print!("Comparing: {:?} and {:?} =>", a, b);
                        if let Some(a_rules) = before_rules.get(a) {
                            if a_rules.contains(b) {
                                println!("LESS");
                                return Ordering::Less;
                            }
                        }

                        if let Some(b_rules) = before_rules.get(b) {
                            if b_rules.contains(a) {
                                println!("GREATER");
                                return Ordering::Greater;
                            }
                        }
                        println!("Equal");
                        Ordering::Equal
                    })
                    .collect();
                println!("ORDER: {:?}", sorted);
                sorted[sorted.len() / 2]
            }
        })
        .sum();
    Ok(total)
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

    const EXAMPLE: &str = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
"#;

    #[test]
    fn test_day2_part1() {
        let data = parse(EXAMPLE);
        assert_eq!(part1(&data).unwrap(), 143);
    }

    #[test]
    fn test_day2_part2() {
        let data = parse(EXAMPLE);
        assert_eq!(part2(&data).unwrap(), 123);
    }
}
