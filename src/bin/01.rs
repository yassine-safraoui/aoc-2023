use anyhow::Result;
use log::{LevelFilter, error, info, warn};
use std::fs::File;
use std::iter;

advent_of_code::solution!(1);

pub struct Input {
    moves: Vec<isize>,
}

pub fn parse_input(input: &str) -> Input {
    let moves = input
        .lines()
        .map(|line| {
            let sign = match line.as_bytes()[0] as char {
                'L' => -1,
                'R' => 1,
                other => {
                    error!("no L no R {other}");
                    0
                }
            };

            line[1..].parse::<isize>().unwrap() * sign
        })
        .collect();
    Input { moves }
}

pub fn part_one(input: &Input) -> Option<u64> {
    if let Err(e) = env_logger::builder()
        .filter_level(LevelFilter::Info)
        .try_init()
    {
        info!("env_logger already initialized")
    }
    let mut count = 0;
    iter::once(50)
        .chain(input.moves.iter().copied())
        .reduce(|acc, e| {
            let result = (((acc + e) % 100) + 100) % 100;
            info!("acc: {acc}, element: {e}");
            if result == 0 {
                count += 1;
                info!("bingo, result is 0!")
            }
            result
        })
        .unwrap();
    Some(count)
}

pub fn part_two(input: &Input) -> Option<u64> {
    let log_file = Box::new(File::create("log.txt").expect("Can't create file"));
    if let Err(_) = env_logger::builder()
        .target(env_logger::Target::Pipe(log_file))
        .filter_level(LevelFilter::Info)
        .try_init()
    {
        info!("env_logger already initialized")
    }
    let mut count: u64 = 0;
    iter::once(50)
        .chain(input.moves.iter().copied())
        .reduce(|acc, mut e| {
            info!("element: {e}, acc: {acc}");
            if e.abs() > 100 {
                count += e.abs() as u64 / 100;
                e %= 100;
            }
            let result = acc + e;
            if result <= 0 || result >= 100 {
                count += 1;
                info!("bingo, result traversed 0!")
            }
            let result = (((result) % 100) + 100) % 100;
            result
        })
        .unwrap();
    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = parse_input(&advent_of_code::template::read_file("examples", DAY));
        let result = part_one(&input);
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let input = parse_input(&advent_of_code::template::read_file("examples", DAY));
        let result = part_two(&input);
        assert_eq!(result, Some(6));
    }
}
