use anyhow::*;
use aoc_2024_in_rust::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "08";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

struct Antenna {
    freq: char,
    x: i32,
    y: i32,
}

impl Antenna {
    fn new(freq: char, x: i32, y: i32) -> Self {
        Self { freq, x, y }
    }
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut antennas: Vec<Antenna> = Vec::new();
        let mut antinodes = HashSet::new();

        let input = parse_input(reader);
        let xlen = input[0].len() as i32;
        let ylen = input.len() as i32;

        for x in 0..xlen {
            for y in 0..ylen {
                let freq = input[y as usize][x as usize];
                if freq != '#' && freq != '.' {
                    antennas.push(Antenna::new(freq, x, y));
                }
            }
        }

        for i in 0..antennas.len() {
            for j in 0..antennas.len() {
                if i == j || antennas[i].freq != antennas[j].freq {
                    continue;
                }

                let Antenna { x: x1, y: y1, .. } = antennas[i];
                let Antenna { x: x2, y: y2, .. } = antennas[j];
                let (xdiff, ydiff) = (x2 - x1, y2 - y1);

                let (xpos, ypos) = (x1 - xdiff, y1 - ydiff);

                if xpos >= 0 && xpos < xlen && ypos >= 0 && ypos < ylen {
                    antinodes.insert((xpos, ypos));
                }
            }
        }

        Ok(antinodes.len())
    }

    assert_eq!(14, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {result}");
    // Result = 396
    //endregion

    //region Part 2
    /*
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut answer = 0;

        Ok(answer)
    }

    assert_eq!(34, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {result}");
    */
    //endregion

    Ok(())
}

fn parse_input<R: BufRead>(reader: R) -> Vec<Vec<char>> {
    reader
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect()
}
