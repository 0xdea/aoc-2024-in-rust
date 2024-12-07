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

        let operators = &['+', '*'];

        for line in reader.lines() {
            let line = line?;

            let v: Vec<usize> = line
                .replace(":", "")
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();

            let operators_num = v.len() - 2;
            let perms = permutations(operators, operators_num);

            for perm in perms {
                let mut res = *&v[1..].first().unwrap().to_owned();

                for (i, operand) in v[1..].iter().enumerate().clone() {
                    if res > *v.first().unwrap() {
                        break;
                    }

                    match perm.chars().nth(i) {
                        Some('+') => res += operand,
                        Some('*') => res *= operand,
                        c => panic!("{c:?}"),
                    }
                }

                if res == *v.first().unwrap() {
                    answer += res;
                    break;
                }
            }
        }

        Ok(answer)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(0, part1(BufReader::new(TEST.as_bytes()))?);

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

fn permutations(symbols: &[char], length: usize) -> Vec<String> {
    std::iter::repeat(symbols.iter())
        .take(length)
        .multi_cartesian_product()
        .map(|v| v.into_iter().collect())
        .collect()
}
