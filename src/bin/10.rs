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
        let grid = parse_input(reader);

        Ok(trailheads(&grid)
            .map(|(x, y)| reachable_peaks(&grid, x, y, 0).len())
            .sum())
    }

    assert_eq!(36, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {result}");
    // Result = 841
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let grid = parse_input(reader);

        Ok(trailheads(&grid)
            .map(|(x, y)| unique_paths(&grid, x, y, 0))
            .sum())
    }

    assert_eq!(81, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {result}");
    // Result = 1875
    //endregion

    Ok(())
}

#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss
)]
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

#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss
)]
fn trailheads(grid: &[Vec<i32>]) -> impl Iterator<Item = (i32, i32)> + '_ {
    let width = grid[0].len() as i32;
    let height = grid.len() as i32;

    (0..width)
        .cartesian_product(0..height)
        .filter(|&(x, y)| grid[y as usize][x as usize] == 0)
}

#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss
)]
fn reachable_peaks(grid: &[Vec<i32>], x: i32, y: i32, exp: i32) -> HashSet<(i32, i32)> {
    let width = grid[0].len() as i32;
    let height = grid.len() as i32;
    let within_bounds = |x, y, width, height| x >= 0 && y >= 0 && x < width && y < height;

    if !within_bounds(x, y, width, height) || grid[y as usize][x as usize] != exp {
        return HashSet::new();
    }

    if grid[y as usize][x as usize] == 9 {
        return HashSet::from_iter([(x, y)]);
    }

    let mut set = reachable_peaks(grid, x - 1, y, exp + 1);
    set.extend(reachable_peaks(grid, x + 1, y, exp + 1));
    set.extend(reachable_peaks(grid, x, y - 1, exp + 1));
    set.extend(reachable_peaks(grid, x, y + 1, exp + 1));

    set
}

#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss
)]
fn unique_paths(grid: &[Vec<i32>], x: i32, y: i32, exp: i32) -> usize {
    let width = grid[0].len() as i32;
    let height = grid.len() as i32;
    let within_bounds = |x, y, width, height| x >= 0 && y >= 0 && x < width && y < height;

    if !within_bounds(x, y, width, height) || grid[y as usize][x as usize] != exp {
        return 0;
    }

    if grid[y as usize][x as usize] == 9 {
        return 1;
    }

    let mut count = 0;
    count += unique_paths(grid, x - 1, y, exp + 1);
    count += unique_paths(grid, x + 1, y, exp + 1);
    count += unique_paths(grid, x, y - 1, exp + 1);
    count += unique_paths(grid, x, y + 1, exp + 1);

    count
}
