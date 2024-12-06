use anyhow::*;
use aoc_2024_in_rust::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
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
        let mut v1: Vec<i32> = vec![];
        let mut v2: Vec<i32> = vec![];

        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.split_whitespace().collect();
            assert_eq!(parts.len(), 2);
            v1.push(parts[0].parse()?);
            v2.push(parts[1].parse()?);
        }

        v1.sort_unstable();
        v2.sort_unstable();

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
    println!("Result = {result}");
    // Result = 2066446
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut v1: Vec<usize> = vec![];
        let mut v2: Vec<usize> = vec![];

        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.split_whitespace().collect();
            assert_eq!(parts.len(), 2);
            v1.push(parts[0].parse()?);
            v2.push(parts[1].parse()?);
        }

        let counts = count_occurrences(&v1, &v2);
        let answer = v1
            .iter()
            .fold(0, |acc, &x| acc + x * counts.get(&x).unwrap());

        Ok(answer)
    }

    assert_eq!(31, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {result}");
    Ok(())
    // Result = 24931009
    //endregion
}

fn count_occurrences(v1: &[usize], v2: &[usize]) -> HashMap<usize, usize> {
    let mut counts = HashMap::new();
    for &n in v1 {
        let count = v2.iter().filter(|&&x| x == n).count();
        counts.insert(n, count);
    }
    counts
}
