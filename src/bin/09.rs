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

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Block {
    FileId(i32),
    Free(i32),
}

#[derive(Debug)]
struct Disk(BTreeMap<Block, i32>);

impl Disk {
    fn new<R: BufRead>(mut reader: R) -> Self {
        let mut disk = BTreeMap::new();

        let mut buf = String::new();
        reader.read_line(&mut buf);

        for (i, c) in buf.trim().chars().enumerate() {
            let size = c.to_digit(10).unwrap() as i32;

            if i % 2 == 0 {
                disk.insert(Block::FileId(i as i32 / 2), size);
            } else {
                disk.insert(Block::Free(i as i32 / 2), size);
            }
        }

        Self(disk)
    }
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(mut reader: R) -> Result<usize> {
        let disk = Disk::new(reader);
        dbg!(&disk);

        disk.0.keys().filter(|k| **)

        for ()

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
