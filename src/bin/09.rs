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

#[derive(Debug)]
struct Blocks(Vec<Option<i32>>);

impl Blocks {
    fn new<R: BufRead>(mut reader: R) -> Self {
        let mut input = String::new();
        reader.read_line(&mut input);

        let mut blocks = vec![];

        for (i, c) in input.trim().chars().enumerate() {
            let size = c.to_digit(10).unwrap() as i32;

            let block = if i % 2 == 0 { Some(i as i32 / 2) } else { None };

            blocks.extend((0..size).map(|_| block));
        }

        Self(blocks)
    }
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(mut reader: R) -> Result<usize> {
        //let disk = Disk::new(reader);
        //dbg!(&disk);

        let blocks = Blocks::new(&mut reader);
        dbg!(&blocks);

        Ok(0)
    }

    assert_eq!(1928, part1(BufReader::new(TEST.as_bytes()))?);

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
