use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "03";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
";
const TEST2: &str = "\
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
";

const RE: &str = r"mul\((\d+),(\d+)\)";
const RE_ANCHORED: &str = r"^mul\((\d+),(\d+)\)";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let re = Regex::new(RE)?;
        let mut answer = 0;

        for line in reader.lines() {
            let line = line?;

            let caps = re.captures_iter(&line);
            for cap in caps {
                let x: usize = cap.get(1).unwrap().as_str().parse()?;
                let y: usize = cap.get(2).unwrap().as_str().parse()?;
                answer += x * y;
            }
        }

        Ok(answer)
    }

    assert_eq!(161, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {result}");
    // Result = 182619815
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let re = Regex::new(RE_ANCHORED)?;
        let mut answer = 0;
        let mut enabled = true;

        for line in reader.lines() {
            let line = line?;

            for i in 0..line.len() {
                if line[i..].starts_with(r"don't()") {
                    enabled = false;
                } else if line[i..].starts_with(r"do()") {
                    enabled = true;
                } else if line[i..].starts_with(r"mul(") && enabled {
                    if let Some(cap) = re.captures(&line[i..]) {
                        let x: usize = cap.get(1).unwrap().as_str().parse()?;
                        let y: usize = cap.get(2).unwrap().as_str().parse()?;
                        answer += x * y;
                    }
                }
            }
        }

        Ok(answer)
    }

    assert_eq!(48, part2(BufReader::new(TEST2.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {result}");
    // Result = 80747545
    //endregion

    Ok(())
}
