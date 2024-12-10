use anyhow::*;
use aoc_2024_in_rust::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "10";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let map = parse_input(reader);

        Ok(trailheads(&map)
            .map(|(x, y)| reachable_peaks(&map, x, y, 0).len())
            .sum())
    }

    assert_eq!(36, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {result}");
    // Result = 841
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

fn parse_input<R: BufRead>(reader: R) -> Vec<Vec<i32>> {
    reader
        .lines()
        .map(|l| {
            l.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect()
}

fn trailheads(map: &Vec<Vec<i32>>) -> impl Iterator<Item = (i32, i32)> + '_ {
    let width = map[0].len() as i32;
    let height = map.len() as i32;

    (0..width)
        .cartesian_product(0..height)
        .filter(|&(x, y)| map[y as usize][x as usize] == 0)
}

fn reachable_peaks(map: &Vec<Vec<i32>>, x: i32, y: i32, exp: i32) -> HashSet<(i32, i32)> {
    let width = map[0].len() as i32;
    let height = map.len() as i32;
    let within_bounds = |x, y, width, height| x >= 0 && y >= 0 && x < width && y < height;

    if !within_bounds(x, y, width, height) || map[y as usize][x as usize] != exp {
        return HashSet::new();
    }

    if map[y as usize][x as usize] == 9 {
        return HashSet::from_iter([(x, y)]);
    }

    let mut set = reachable_peaks(map, x - 1, y, exp + 1);
    set.extend(reachable_peaks(map, x + 1, y, exp + 1));
    set.extend(reachable_peaks(map, x, y - 1, exp + 1));
    set.extend(reachable_peaks(map, x, y + 1, exp + 1));

    set
}
