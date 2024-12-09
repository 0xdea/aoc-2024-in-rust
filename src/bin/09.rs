use anyhow::*;
use aoc_2024_in_rust::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::{Deref, DerefMut};

const DAY: &str = "09";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
2333133121414131402
";

type Block = Option<i32>;

#[derive(Debug)]
struct Blocks(Vec<(Block, i32)>);

impl Deref for Blocks {
    type Target = Vec<(Block, i32)>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Blocks {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss
)]
impl Blocks {
    fn new<R: BufRead>(mut reader: R) -> Self {
        let mut input = String::new();
        reader
            .read_line(&mut input)
            .expect("error processing input");

        let mut blocks = vec![];
        for (i, c) in input.trim().chars().enumerate() {
            let size = c.to_digit(10).unwrap() as i32;
            let block = if i % 2 == 0 {
                (Some(i as i32 / 2), size)
            } else {
                (None, size)
            };
            blocks.extend((0..size).map(|_| block));
        }

        Self(blocks)
    }

    fn checksum(&self) -> usize {
        self.iter()
            .map_while(|&block| block.0)
            .enumerate()
            .map(|(i, id)| i * id as usize)
            .sum()
    }
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(mut reader: R) -> Result<usize> {
        // Process input disk
        let mut blocks = Blocks::new(&mut reader);
        // dbg!(&blocks);

        // Defrag input disk
        let mut left = 0;
        let mut right = blocks.len() - 1;
        while left < right {
            if blocks[left].0.is_some() {
                left += 1;
            } else if blocks[right].0.is_none() {
                right -= 1;
            } else {
                blocks.swap(left, right);
            }
        }

        // Calculate checksum
        Ok(blocks.checksum())
    }

    assert_eq!(1928, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {result}");
    // Result = 6398252054886
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(mut reader: R) -> Result<usize> {
        // Process input disk
        let mut blocks = Blocks::new(&mut reader);
        // dbg!(&blocks);

        //let mut left = 0;
        let mut right = blocks.len() - 1;

        while right >= 2 {
            for left in (1..right).step_by(2) {
                println!("left = {left}");
                let left_len = blocks[left].1;
                let right_len = blocks[right].1;
                if left_len >= right_len {
                    blocks.swap(left, right);
                    blocks[right].1 = right_len;
                    blocks.insert(left, (None, 0));
                    blocks.insert(left + 2, (None, left_len - right_len));
                    right += 2;
                    break;
                }
            }
            right -= 2;
            println!("right = {right}");
        }

        /*
        // Defrag input disk

        while left < right {
            if blocks[left].0.is_some() {
                left += 1;
            } else if blocks[right].0.is_none() {
                right -= 1;
            } else {
                blocks.swap(left, right);
            }
        }
         */

        // Calculate checksum
        Ok(blocks.checksum())
    }

    assert_eq!(2858, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {result}");
    //endregion

    Ok(())
}
