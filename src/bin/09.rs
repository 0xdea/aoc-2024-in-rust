use anyhow::*;
use aoc_2024_in_rust::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "09";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
2333133121414131402
";

type Block = Option<i32>;

#[derive(Debug)]
struct Blocks(Vec<Block>);

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
            let block = if i % 2 == 0 { Some(i as i32 / 2) } else { None };
            blocks.extend((0..size).map(|_| block));
        }

        Self(blocks)
    }

    fn get(&self, index: usize) -> Option<Block> {
        self.0.get(index).copied()
    }

    fn get_mut(&mut self, index: usize) -> Option<&mut Block> {
        self.0.get_mut(index)
    }

    fn remove(&mut self, index: usize) -> Block {
        self.0.remove(index)
    }

    fn iter(&self) -> impl DoubleEndedIterator<Item = Block> + ExactSizeIterator + '_ {
        self.0.iter().copied()
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn checksum(&self) -> usize {
        self.iter()
            .map_while(|block| block)
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
        let mut i = 0;
        let mut j = blocks.len() - 1;
        while i < j {
            if blocks.get(i).unwrap().is_some() {
                i += 1;
            } else {
                *blocks.get_mut(i).unwrap() = blocks.remove(j);
                j = blocks
                    .iter()
                    .enumerate()
                    .rev()
                    .find_map(|(idx, block)| block.map(|_| idx))
                    .unwrap_or_default();
            }
        }

        Ok(blocks.checksum())
    }

    assert_eq!(1928, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {result}");
    // Result = 6398252054886
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
