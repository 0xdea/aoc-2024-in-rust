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
        dbg!(&stones);

        for _ in 0..25 {
            stones = blink(&stones);
        }

        Ok(stones.values().sum())
    }

    assert_eq!(55312, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {result}");
    //endregion

    //region Part 2
    /*
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut answer = 0;

        Ok(answer)
    }

    assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {result}");
    */
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
    let mut result = HashMap::new();
    for (stone, count) in stones {
        match *stone {
            0 => *result.entry(1).or_default() += count,
            _ => {
                let digits = format!("{}", *stone).len() as u32;
                if digits % 2 == 0 {
                    *result.entry(stone % 10_isize.pow(digits / 2)).or_default() += count;
                    *result.entry(stone / 10_isize.pow(digits / 2)).or_default() += count;
                } else {
                    *result.entry(stone * 2024).or_default() += count;
                }
            }
        }
    }
    result
}
