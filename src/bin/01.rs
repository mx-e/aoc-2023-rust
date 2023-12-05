advent_of_code::solution!(1);
use regex::Regex;
#[macro_use] extern crate lazy_static;

lazy_static! {
    static ref REG: regex::Regex = Regex::new(r"(\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    static ref REG_REV: regex::Regex = Regex::new(r"(\d|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin)").unwrap();
}

fn replace_with_digit(matched_str: &str) -> &str {
    match matched_str {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        _ => matched_str,
    }
}

fn replace_with_reverse_digit(matched_str: &str) -> &str {
    match matched_str {
        "eno" => "1",
        "owt" => "2",
        "eerht" => "3",
        "ruof" => "4",
        "evif" => "5",
        "xis" => "6",
        "neves" => "7",
        "thgie" => "8",
        "enin" => "9",
        _ => matched_str,
    }
}

fn get_first_last_digit_concat_sum(line: &str) -> u32 {
    let first = line.chars().find(|c| c.is_ascii_digit()).unwrap();
    let last = line.chars().rev().find(|c| c.is_ascii_digit()).unwrap();
    format!("{}{}", first, last).parse::<u32>().unwrap()
}

fn get_first_last_number_concat_sum(line: &str) -> u32 {
    let rev_line = line.chars().rev().collect::<String>();
    let first = replace_with_digit(REG.find(line).unwrap().as_str());
    let last = replace_with_reverse_digit(REG_REV.find(&rev_line).unwrap().as_str());
    format!("{}{}", first, last).parse::<u32>().unwrap()
}

pub fn part_one(input: &str) -> Option<u32> {
    let result = input.lines().map(get_first_last_digit_concat_sum).sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let result = input.lines().map(get_first_last_number_concat_sum).sum();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result.unwrap(), 220);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result.unwrap(), 281);
    }
}
