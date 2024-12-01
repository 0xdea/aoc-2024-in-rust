use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<i32> {
        let mut v1 = vec![];
        let mut v2 = vec![];

        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() != 2 {
                panic!();
            }
            let n1: i32 = parts[0].parse()?;
            let n2: i32 = parts[1].parse()?;
            v1.push(n1);
            v2.push(n2);
        }

        v1.sort();
        v2.sort();

        let answer = v1
            .iter()
            .zip(v2.iter())
            .map(|(n1, n2)| (n1 - n2).abs())
            .sum();

        Ok(answer)
    }

    assert_eq!(11, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    // Result = 2066446
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut v1 = vec![];
        let mut v2 = vec![];

        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() != 2 {
                panic!();
            }
            let n1: i32 = parts[0].parse()?;
            let n2: i32 = parts[1].parse()?;
            v1.push(n1);
            v2.push(n2);
        }

        let counts = count_occurrences(&v1, &v2);

        let answer = v1.iter().fold(0, |acc, &x| {
            acc + x as usize * counts.get(&x).unwrap_or(&0usize)
        });

        Ok(answer)
    }

    assert_eq!(31, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    Ok(())
    // Result = 24931009
    //endregion
}

fn count_occurrences(v1: &Vec<i32>, v2: &Vec<i32>) -> HashMap<i32, usize> {
    let mut counts = HashMap::new();
    for &n in v1 {
        let count = v2.iter().filter(|&&x| x == n).count();
        counts.insert(n, count);
    }
    counts
}
