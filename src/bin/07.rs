use anyhow::*;
use aoc_2024_in_rust::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "07";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut answer = 0;

        for line in reader.lines() {
            let line = line?;
            let parts: Vec<_> = line.split(':').collect();
            assert_eq!(parts.len(), 2);

            let result: usize = parts[0].parse()?;
            let operands: Vec<usize> = parts[1]
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();

            let perms = calculate_permutations(&['+', '*'], operands.len() - 1);
            for perm in perms {
                let mut res = *operands.first().unwrap();

                for (i, operand) in operands[1..].iter().enumerate() {
                    if res > result {
                        break;
                    }

                    match perm.chars().nth(i) {
                        Some('+') => res += operand,
                        Some('*') => res *= operand,
                        _ => panic!(),
                    }
                }

                if res == result {
                    answer += res;
                    break;
                }
            }
        }

        Ok(answer)
    }

    assert_eq!(3749, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {result}");
    // Result = 850435817339
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut answer = 0;

        for line in reader.lines() {
            let line = line?;
            let parts: Vec<_> = line.split(':').collect();
            assert_eq!(parts.len(), 2);

            let result: usize = parts[0].parse()?;
            let operands: Vec<usize> = parts[1]
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();

            let perms = calculate_permutations(&['+', '*', '|'], operands.len() - 1);
            for perm in perms {
                let mut res = *operands.first().unwrap();

                for (i, operand) in operands[1..].iter().enumerate() {
                    if res > result {
                        break;
                    }

                    match perm.chars().nth(i) {
                        Some('+') => res += operand,
                        Some('*') => res *= operand,
                        Some('|') => res = format!("{res}{operand}").parse()?,
                        _ => panic!(),
                    }
                }

                if res == result {
                    answer += res;
                    break;
                }
            }
        }

        Ok(answer)
    }

    assert_eq!(11387, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {result}");
    // Result = 104824810233437
    //endregion

    Ok(())
}

fn calculate_permutations(chars: &[char], len: usize) -> Vec<String> {
    std::iter::repeat(chars.iter())
        .take(len)
        .multi_cartesian_product()
        .map(|v| v.into_iter().collect())
        .collect()
}
