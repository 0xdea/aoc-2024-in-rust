use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "05";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
47|53
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
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut answer = 0;
        let mut rules = vec![];
        let mut pages_lines = vec![];
        let mut reading_pages = false;

        for line in reader.lines() {
            let line = line?;

            if line.trim().is_empty() {
                reading_pages = true;
            } else {
                if reading_pages {
                    pages_lines.push(line);
                } else {
                    rules.push(line);
                }
            }
        }

        for line in pages_lines {
            let pages: Vec<usize> = line.split(',').map(|x| x.parse().unwrap()).collect();

            if is_correct_order(&pages, &rules) {
                let mid = pages.get(pages.len() / 2).unwrap();
                answer += mid;
            }
        }

        Ok(answer)
    }

    assert_eq!(143, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {result}");
    // Result = 3608
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut answer = 0;
        let mut rules = vec![];
        let mut pages_lines = vec![];
        let mut reading_pages = false;

        for line in reader.lines() {
            let line = line?;

            if line.trim().is_empty() {
                reading_pages = true;
            } else {
                if reading_pages {
                    pages_lines.push(line);
                } else {
                    rules.push(line);
                }
            }
        }

        for line in pages_lines {
            let mut pages: Vec<usize> = line.split(',').map(|x| x.parse().unwrap()).collect();

            if !is_correct_order(&pages, &rules) {
                let mut swapped = true;

                while swapped {
                    swapped = false;
                    for i in 0..pages.len() - 1 {
                        let tmp = format!("{}|{}", pages[i + 1], pages[i]);
                        if rules.contains(&tmp) {
                            pages.swap(i, i + 1);
                            swapped = true;
                        }
                    }
                }

                let mid = pages.get(pages.len() / 2).unwrap();
                answer += mid;
            }
        }

        Ok(answer)
    }

    assert_eq!(123, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {result}");
    // Result = 4922
    //endregion

    Ok(())
}

fn is_correct_order(pages: &[usize], rules: &[String]) -> bool {
    for (i, page) in pages.iter().enumerate() {
        for j in 0..i {
            let tmp = pages.get(j).unwrap();
            let test = format!("{page}|{tmp}");
            if rules.contains(&&test) {
                return false;
            }
        }
    }
    true
}
