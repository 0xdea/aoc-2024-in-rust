use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "06";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    const fn new() -> Self {
        Self::Up
    }

    const fn invert(self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Right => Self::Left,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
        }
    }

    const fn turn_right(self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Right => Self::Up,
            Self::Down => Self::Right,
            Self::Left => Self::Down,
        }
    }
}

struct Position {
    x: i32,
    y: i32,
}

impl Position {
    const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let input = parse_input(reader);
        Ok(color_path(input).unwrap())
    }

    assert_eq!(41, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {result}");
    // Result = 4967
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut answer = 0;

        let input = parse_input(reader);
        let xlen = input[0].len();
        let ylen = input.len();

        // Brute force approach;)
        for x in 0..xlen {
            for y in 0..ylen {
                let mut test_input = input.clone();
                // The new obstruction can't be placed at the guard's starting position
                if input[y][x] != '^' {
                    test_input[y][x] = '#';
                    if color_path(test_input).is_none() {
                        answer += 1;
                    }
                }
            }
        }

        Ok(answer)
    }

    assert_eq!(6, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {result}");
    // Result = 1789
    //endregion

    Ok(())
}

fn parse_input<R: BufRead>(reader: R) -> Vec<Vec<char>> {
    reader
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect()
}

fn move_guard(x: &mut i32, y: &mut i32, dir: Direction) {
    match dir {
        Direction::Up => *y -= 1,
        Direction::Right => *x += 1,
        Direction::Down => *y += 1,
        Direction::Left => *x -= 1,
    };
}

#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss
)]
fn color_path(mut input: Vec<Vec<char>>) -> Option<usize> {
    let mut count = 1;

    let xlen = input[0].len();
    let ylen = input.len();

    let mut pos = Position::new(0, 0);
    let mut dir = Direction::new();
    let mut visited = HashSet::new();

    #[allow(clippy::needless_range_loop)]
    for x in 0..xlen {
        for y in 0..ylen {
            if input[y][x] == '^' {
                input[y][x] = 'X';
                (pos.x, pos.y) = (x as i32, y as i32);
            }
        }
    }

    loop {
        // Return None if guard gets stuck in a loop
        if visited.contains(&(pos.x, pos.y, dir)) {
            return None;
        }

        // Otherwise keep moving
        visited.insert((pos.x, pos.y, dir));
        move_guard(&mut pos.x, &mut pos.y, dir);

        // Exit loop if guard leaves the area
        if pos.x < 0 || pos.y < 0 || pos.x >= xlen as i32 || pos.y >= ylen as i32 {
            break;
        }

        #[allow(clippy::match_on_vec_items)]
        match input[pos.y as usize][pos.x as usize] {
            '#' => {
                dir = dir.invert();
                move_guard(&mut pos.x, &mut pos.y, dir);
                dir = dir.turn_right();
            }
            c if c != 'X' => {
                count += 1;
                input[pos.y as usize][pos.x as usize] = 'X';
            }
            _ => (),
        }
    }

    Some(count)
}
