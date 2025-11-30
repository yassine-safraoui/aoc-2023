use log::{debug, error, info, warn};
use std::time::Instant;
pub struct Day();

advent_of_code::solution!(1);

pub struct Input {
    lines: Vec<String>,
}

pub fn parse_input(input: &str) -> Input {
    // let start = Instant::now();
    let mut lines = vec![];
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        lines.push(line.to_string())
    }
    // println!("Parsing input took: {:?}", start.elapsed());
    Input { lines }
}

pub fn part_one(input: &Input) -> Option<u64> {
    let mut result: u64 = 0;
    for line in &input.lines {
        if !line.is_ascii() {
            warn!("Found line containing non ascii characters, line: {}", line);
            continue;
        }
        let line = line.as_bytes();
        let mut first_digit: Option<u8> = None;
        let mut last_digit: Option<u8> = None;
        for ch in line {
            if ch.is_ascii_digit() {
                let digit = *ch - b'0';
                if first_digit.is_none() {
                    first_digit = Some(digit);
                }
                last_digit = Some(digit);
            }
        }
        if first_digit.is_none() {
            continue;
        }
        if last_digit.is_none() {
            continue;
        }
        result += (first_digit.unwrap() * 10 + last_digit.unwrap()) as u64;
    }
    Some(result)
}

struct Digit {
    digit: usize,
    index: usize,
}

pub fn part_two(input: &Input) -> Option<u64> {
    let spelled_digits: [String; 9] = [
        String::from("one"),
        String::from("two"),
        String::from("three"),
        String::from("four"),
        String::from("five"),
        String::from("six"),
        String::from("seven"),
        String::from("eight"),
        String::from("nine"),
    ];
    let mut result: u64 = 0;
    for line in &input.lines {
        if !line.is_ascii() {
            warn!("Found line containing non ascii characters, line: {}", line);
            continue;
        }
        let string_line = line.clone();

        let mut first_spelled_digit: Option<Digit> = None;
        let mut last_spelled_digit: Option<Digit> = None;
        for (index, spelled_digit) in spelled_digits.iter().enumerate() {
            let digit = index + 1;
            let mut occurrences = line.match_indices(spelled_digit);
            let first_occurrence = occurrences.next();
            if first_occurrence.is_none() {
                continue;
            }
            let first_occurrence = first_occurrence.unwrap();
            let mut last_occurrence = first_occurrence;
            while let Some(item) = occurrences.next() {
                last_occurrence = item;
            }
            if first_spelled_digit.is_none()
                || first_spelled_digit.as_ref().unwrap().index > first_occurrence.0
            {
                first_spelled_digit = Some(Digit {
                    digit,
                    index: first_occurrence.0,
                })
            }

            if last_spelled_digit.is_none()
                || last_spelled_digit.as_ref().unwrap().index < last_occurrence.0
            {
                last_spelled_digit = Some(Digit {
                    digit,
                    index: last_occurrence.0,
                })
            }
        }

        let line = Vec::from(line.as_bytes());
        let mut first_digit: Option<u32> = None;
        let mut last_digit: Option<u32> = last_spelled_digit.as_ref().map(|t| t.digit as u32);
        for (index, ch) in line.iter().enumerate() {
            if ch.is_ascii_digit() {
                let digit = (*ch as char).to_digit(10);
                if digit.is_none() {
                    error!("Failed to parse digit in line, line: {}", string_line);
                    continue;
                }
                if (first_spelled_digit.as_ref().is_none()
                    || first_spelled_digit.as_ref().unwrap().index > index)
                    && first_digit.is_none()
                {
                    debug!(
                        "Found first numerical digit in line, line: {}, index: {}",
                        string_line, index
                    );
                    first_digit = Some(digit.unwrap());
                }
                if last_spelled_digit.as_ref().is_none()
                    || last_spelled_digit.as_ref().unwrap().index < index
                {
                    debug!(
                        "Found numerical digit in line, line: {}, index: {}",
                        string_line, index
                    );
                    last_digit = Some(digit.unwrap());
                }
            }
        }
        if first_digit.is_none() && first_spelled_digit.is_some() {
            first_digit = first_spelled_digit.as_ref().map(|t| t.digit as u32)
        }
        if first_digit.is_none() {
            error!("Invalid first digit in line, line: {}", string_line);
            break;
        }
        if last_digit.is_none() {
            error!("Invalid last digit in line, line: {}", string_line);
            break;
        }
        result += (first_digit.unwrap() * 10 + last_digit.unwrap()) as u64;
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = parse_input(&advent_of_code::template::read_file("examples", DAY));
        let result = part_one(&input);
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let input = parse_input(&advent_of_code::template::read_file("examples", DAY));
        let result = part_two(&input);
        assert_eq!(result, None);
    }

    #[test]
    fn custom_test() {
        // let input = parse_input(&advent_of_code::template::read_file("examples", DAY));
        // debug!("input length: {}", input.lines.len());
        let input = Input {
            lines: vec![String::from("zoe8threeifj9")],
        };
        let result = part_two(&input);
        if result.is_some() {
            println!("result: {}", result.unwrap());
        }
    }
}
