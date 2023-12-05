advent_of_code::solution!(3);

use regex::{Regex, Match};
#[macro_use] extern crate lazy_static;

lazy_static! {
    static ref REG_NUM: regex::Regex = Regex::new(r"(\d+)").unwrap();
    static ref REG_SYM: regex::Regex = Regex::new(r"[^A-Za-z0-9.]").unwrap();
}
const LINE_LENGTH: i32 = 140;
const DIR_OFFSET: [i32; 8] = [
    -LINE_LENGTH-1, -LINE_LENGTH, -LINE_LENGTH+1,-1, 1,LINE_LENGTH-1, LINE_LENGTH, LINE_LENGTH+1
];
const MAX_NUM_LENGTH: usize = 5;

fn get_digit_mask(line: &str, digit_pos: usize, m_start: usize, m_end: usize) -> [i32; 8] {
    let mut mask = [1; 8];
    if digit_pos > m_start {
        mask[0] = 0;
        mask[3] = 0;
        mask[5] = 0;
    }
    if digit_pos < m_end -1 {
        mask[2] = 0;
        mask[4] = 0;
        mask[7] = 0;
    }
    if digit_pos < LINE_LENGTH as usize{
        mask[0] = 0;
        mask[1] = 0;
        mask[2] = 0;
    }
    if digit_pos >= line.len() - LINE_LENGTH as usize {
        mask[5] = 0;
        mask[6] = 0;
        mask[7] = 0;
    }
    if digit_pos % LINE_LENGTH as usize == 0 {
        mask[0] = 0;
        mask[3] = 0;
        mask[5] = 0;
    }
    if digit_pos % LINE_LENGTH as usize == (LINE_LENGTH as usize - 1) {
        mask[2] = 0;
        mask[4] = 0;
        mask[7] = 0;
    }
    mask
}

fn check_surrounding_str(line: &str, d_start: usize, d_end: usize) -> u32 {
    let num: u32 = line[d_start..d_end].parse().unwrap();
    for d_pos in d_start..d_end {
        let mask = get_digit_mask(line, d_pos, d_start, d_end);
        for (i, offset) in DIR_OFFSET.iter().enumerate() {
            if mask[i] == 1 {
                let idx = (d_pos as i32 + offset) as usize;
                let c = line[idx..idx+1].chars().next().unwrap();
                if !c.is_alphanumeric() && c != '.' {
                    return num;
                }
                
            }
        }
    };
    0
}

fn get_sym_mask(line: &str, sym_pos: usize) -> [i32; 8] {
    let mut mask = [1; 8];
    if sym_pos < LINE_LENGTH as usize{
        mask[0] = 0;
        mask[1] = 0;
        mask[2] = 0;
    }
    if sym_pos >= line.len() - LINE_LENGTH as usize {
        mask[5] = 0;
        mask[6] = 0;
        mask[7] = 0;
    }
    if sym_pos % LINE_LENGTH as usize == 0 {
        mask[0] = 0;
        mask[3] = 0;
        mask[5] = 0;
    }
    if sym_pos % LINE_LENGTH as usize == (LINE_LENGTH as usize - 1) {
        mask[2] = 0;
        mask[4] = 0;
        mask[7] = 0;
    }
    mask
}

fn check_surr_nums(line: &str, sym_pos: usize, num_matches: &Vec<Match>)-> u64{
    let surr_mask = get_sym_mask(line, sym_pos);
    let surr_idx: Vec<(usize, usize)> = DIR_OFFSET.iter()
    .map(|offset| (sym_pos as i32 + offset) as usize)
    .enumerate()
    .filter(|(i, _)| surr_mask[*i] == 1)
    .collect::<Vec<(usize, usize)>>();
    let matches: Vec<&Match> = num_matches.iter()
    .filter(|m| (sym_pos-LINE_LENGTH as usize - MAX_NUM_LENGTH) < m.start()  && (sym_pos+LINE_LENGTH as usize+MAX_NUM_LENGTH) > m.end())
    .filter(|m| surr_idx.iter().any(
        |(_, idx)| *idx >= m.start() && *idx < m.end())
    )
    .collect();
    if matches.len() == 2 {
        return matches[0].as_str().parse::<u64>().unwrap() * matches[1].as_str().parse::<u64>().unwrap();
    }
    0
}

pub fn part_one(input: &str) -> Option<u32> {
    let single_line_input = input.replace("\n", "");
    let res = REG_NUM.find_iter(&single_line_input)
    .map(|m| check_surrounding_str(&single_line_input, m.start(), m.end()))
    .sum();
    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    let single_line_input = input.replace("\n", "");
    let num_matches = REG_NUM.find_iter(&single_line_input).collect();
    let res: u64 = REG_SYM.find_iter(&single_line_input)
    .map(|m| check_surr_nums(&single_line_input, m.start(), &num_matches)).sum();
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
