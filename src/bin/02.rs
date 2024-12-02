use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "02";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut answer = 0;

        for line in reader.lines() {
            let line = line?;

            let v: Vec<i32> = line
                .split_whitespace()
                .map(|s| s.parse().expect("invalid number"))
                .collect();

            if is_safe(&v) {
                answer += 1;
            }
        }

        Ok(answer)
    }

    assert_eq!(2, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {result}");
    // Result = 624
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut answer = 0;

        for line in reader.lines() {
            let line = line?;

            let v: Vec<i32> = line
                .split_whitespace()
                .map(|s| s.parse().expect("invalid number"))
                .collect();

            if is_safe(&v) || is_safe2(&v) {
                answer += 1;
            }
        }

        Ok(answer)
    }

    assert_eq!(4, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {result}");
    // Result = 658
    //endregion

    Ok(())
}

fn is_safe(v: &[i32]) -> bool {
    if !v.is_sorted() && !v.iter().rev().is_sorted() {
        return false;
    }
    for w in v.windows(2) {
        if (w[0] - w[1]).abs() > 3 || (w[0] == w[1]) {
            return false;
        }
    }
    true
}

// brute force approach;)
fn is_safe2(v: &[i32]) -> bool {
    if is_safe(&v[1..v.len()]) {
        return true;
    }
    for i in 1..v.len() {
        let (left, right) = v.split_at(i);
        let v2 = [left, &right[1..right.len()]].concat();
        if is_safe(&v2) {
            return true;
        }
    }
    false
}
