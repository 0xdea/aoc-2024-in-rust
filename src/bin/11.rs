use anyhow::*;
use aoc_2024_in_rust::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "11";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
125 17
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(mut reader: R) -> Result<usize> {
        let mut stones = parse_input(&mut reader);

        for _ in 0..25 {
            stones = blink(&stones);
        }

        Ok(stones.values().sum())
    }

    assert_eq!(55312, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {result}");
    // Result = 182081
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(mut reader: R) -> Result<usize> {
        let mut stones = parse_input(&mut reader);

        for _ in 0..75 {
            stones = blink(&stones);
        }

        Ok(stones.values().sum())
    }

    //assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {result}");
    // Result = 216318908621637
    //endregion

    Ok(())
}

fn parse_input<R: BufRead>(mut reader: R) -> HashMap<isize, usize> {
    let mut input = String::new();
    reader
        .read_line(&mut input)
        .expect("error processing input");

    input
        .split_whitespace()
        .map(|s| (s.parse().unwrap(), 1))
        .collect()
}

fn blink(stones: &HashMap<isize, usize>) -> HashMap<isize, usize> {
    // Key = stone mark
    // Value = stone count
    let mut result = HashMap::new();

    for (mark, count) in stones {
        // Replace mark 0 with mark 1
        if *mark == 0 {
            *result.entry(1).or_default() += count;
        } else {
            #[allow(clippy::cast_possible_truncation)]
            let digits = format!("{}", *mark).len() as u32;

            // Replace the stone with two stones
            if digits % 2 == 0 {
                *result.entry(mark % 10_isize.pow(digits / 2)).or_default() += count;
                *result.entry(mark / 10_isize.pow(digits / 2)).or_default() += count;

            // Multiply mark by 2024
            } else {
                *result.entry(mark * 2024).or_default() += count;
            }
        }
    }

    result
}
