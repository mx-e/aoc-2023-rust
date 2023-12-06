advent_of_code::solution!(4);
// const N_WINNING: usize = 5;
// const N_CANDS: usize = 8;
const N_WINNING: usize = 10;
const N_CANDS: usize = 25;

fn get_card_n_matches(card: &str) -> u32 {
    let mut winning = [0;N_WINNING];
    let mut cands = [0;N_CANDS];
    let (win_str, cand_str) = card
    .split_once(":")
    .unwrap().1
    .split_once("|")
    .unwrap();

    for (i, num_str) in win_str.split_whitespace().enumerate() {
        winning[i] = num_str.parse::<u32>().unwrap();
    }
    for (i, num_str) in cand_str.split_whitespace().enumerate() {
        cands[i] = num_str.parse::<u32>().unwrap();
    }
    let mut n_matches = 0;
    for win in winning.iter() {
        if cands.contains(win){
            n_matches += 1;
        }
    }
    n_matches
}

fn score_card(n_matches: u32) -> u32 {
    if n_matches == 0 {
        return 0;
    }
    let power = n_matches - 1;
    return 2_i32.pow(power as u32) as u32;
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines()
    .map(get_card_n_matches)
    .map(score_card).sum::<u32>())
}

fn update_card_counts(idx: usize,card_counts: &mut Vec<u32>, n_matches: u32) {
    let n_cards = card_counts.len();
    let current_card_count = card_counts[idx];
    if n_matches > 0 {
        let n_future_cards = n_matches as usize;
        for f_idx in idx+1..idx+n_future_cards+1 {
            if f_idx >= n_cards {
                break;
            }
            card_counts[f_idx] += current_card_count;
        }
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines();
    let mut card_counts = vec![1; lines.count()];
    for (i, line) in input.lines().enumerate() {
        update_card_counts(i, &mut card_counts, get_card_n_matches(line));
    }
    Some(card_counts.iter().sum::<u32>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
