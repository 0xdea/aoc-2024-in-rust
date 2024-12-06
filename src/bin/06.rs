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

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut answer = 1;

        let mut input = parse_input(reader);
        let xlen = input[0].len();
        let ylen = input.len();

        let mut xsave: i32 = 0;
        let mut ysave: i32 = 0;
        let mut dir = Direction::Up;

        let mut visited = HashSet::new();

        for x in 0..xlen {
            for y in 0..ylen {
                if input[y][x] == '^' {
                    input[y][x] = 'X';
                    (xsave, ysave) = (x as i32, y as i32);
                }
            }
        }

        loop {
            if visited.contains(&(xsave, ysave, dir)) {
                break;
            }
            visited.insert((xsave, ysave, dir));
            move_guard(&mut xsave, &mut ysave, dir);

            if ysave < 0 || xsave < 0 || xsave >= xlen as i32 || ysave >= ylen as i32 {
                break;
            }

            match input[ysave as usize][xsave as usize] {
                '#' => {
                    dir = match dir {
                        Direction::Up => Direction::Down,
                        Direction::Right => Direction::Left,
                        Direction::Down => Direction::Up,
                        Direction::Left => Direction::Right,
                    };
                    move_guard(&mut xsave, &mut ysave, dir);
                    dir = match dir {
                        Direction::Up => Direction::Left,
                        Direction::Right => Direction::Up,
                        Direction::Down => Direction::Right,
                        Direction::Left => Direction::Down,
                    }
                }
                c if c != 'X' => {
                    answer += 1;
                    input[ysave as usize][xsave as usize] = 'X';
                }
                _ => (),
            }
        }

        Ok(answer)
    }

    assert_eq!(41, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {result}");
    //endregion

    //region Part 2
    /*
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        Ok(0)
    }

    assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);

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

fn move_guard(x: &mut i32, y: &mut i32, dir: Direction) {
    match dir {
        Direction::Up => *y -= 1,
        Direction::Right => *x += 1,
        Direction::Down => *y += 1,
        Direction::Left => *x -= 1,
    };
}
