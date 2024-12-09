use anyhow::*;
use aoc_2024_in_rust::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "09";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
2333133121414131402
";

struct MyFile {
    size: usize,
    id: i32,
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(mut reader: R) -> Result<usize> {
        let mut input = String::new();
        reader.read_line(&mut input);

        let mut used = BTreeMap::new();
        let mut free = BTreeMap::new();
        let mut pos = 0;

        for (i, c) in input.chars().enumerate() {
            let size = c.to_digit(10).unwrap() as i32;
            if size == 0 {
                continue;
            }

            if i % 2 == 0 {
                used.insert(pos, i);
            } else {
                free.insert(c, i);
            }

            dbg!(&used);
            dbg!(&free);

            let v = if i % 2 == 0 {
                fid += 1;
                fid - 1
            } else {
                -1
            };
            fs1.extend((0..b - b'0').map(|_| (1, v)));
            fs2.push(((b - b'0') as usize, v));
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
