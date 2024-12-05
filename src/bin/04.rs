use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "04";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

#[allow(clippy::too_many_lines)]
fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut answer = 0;
        let is_xmas = |s| s == "XMAS" || s == "SAMX";

        let input = parse_input(reader);
        let xlen = input[0].len();
        let ylen = input.len();

        // Horizontal
        #[allow(clippy::needless_range_loop)]
        for x in 0..xlen - 3 {
            for y in 0..ylen {
                if is_xmas(format!(
                    "{}{}{}{}",
                    input[y][x],
                    input[y][x + 1],
                    input[y][x + 2],
                    input[y][x + 3]
                )) {
                    answer += 1;
                }
            }
        }

        // Vertical
        for x in 0..xlen {
            for y in 0..ylen - 3 {
                if is_xmas(format!(
                    "{}{}{}{}",
                    input[y][x],
                    input[y + 1][x],
                    input[y + 2][x],
                    input[y + 3][x]
                )) {
                    answer += 1;
                }
            }
        }

        // First diagonal
        for x in 0..xlen - 3 {
            for y in 0..ylen - 3 {
                if is_xmas(format!(
                    "{}{}{}{}",
                    input[y][x],
                    input[y + 1][x + 1],
                    input[y + 2][x + 2],
                    input[y + 3][x + 3]
                )) {
                    answer += 1;
                }
            }
        }

        // Second diagonal
        for x in 3..xlen {
            for y in 0..ylen - 3 {
                if is_xmas(format!(
                    "{}{}{}{}",
                    input[y][x],
                    input[y + 1][x - 1],
                    input[y + 2][x - 2],
                    input[y + 3][x - 3]
                )) {
                    answer += 1;
                }
            }
        }

        Ok(answer)
    }

    assert_eq!(18, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {result}");
    // Result = 2462
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut answer = 0;
        let is_x_mas = |s1, s2| (s1 == "MAS" || s1 == "SAM") && (s2 == "MAS" || s2 == "SAM");

        let input = parse_input(reader);
        let xlen = input[0].len();
        let ylen = input.len();

        for x in 0..xlen - 2 {
            for y in 0..ylen - 2 {
                if is_x_mas(
                    format!(
                        "{}{}{}",
                        input[y][x],
                        input[y + 1][x + 1],
                        input[y + 2][x + 2]
                    ),
                    format!(
                        "{}{}{}",
                        input[y][x + 2],
                        input[y + 1][x + 1],
                        input[y + 2][x]
                    ),
                ) {
                    answer += 1;
                }
            }
        }

        Ok(answer)
    }

    assert_eq!(9, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {result}");
    // Result = 1877
    //endregion

    Ok(())
}

fn parse_input<R: BufRead>(reader: R) -> Vec<Vec<char>> {
    reader
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect()
}
