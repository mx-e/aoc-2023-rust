
advent_of_code::solution!(2);
const COLOR_NAMES: [&str; 3] = ["red", "green", "blue"];


fn extract_color_numbers_and_id (line: &str) -> (u32, [u32; 3]) {
    let id_str = line.split(":").next().unwrap();
    let id = id_str[id_str.len()-2..]
        .trim()
        .parse::<u32>()
        .unwrap();
    let mut max_color: [u32; 3] = [0,0,0];
    for (i, color) in COLOR_NAMES.iter().enumerate() {
        for (idx,_) in line.match_indices(color) {
            let color_count =line[idx-3..idx-1]
            .trim()
            .to_string()
            .parse::<u32>()
            .unwrap();
            max_color[i] = max_color[i].max(color_count);
       }
    }
    (id, max_color)
}

pub fn part_one(input: &str) -> Option<u32> {
    const POSSIBLE_COLORS: [u32; 3] = [12, 13, 14];
    let res = input.lines()
    .map(extract_color_numbers_and_id)
    .map(|extracted| {
        let (id, color_numbers) = extracted;
        for (i, color_number) in color_numbers.iter().enumerate() {
            if *color_number > POSSIBLE_COLORS[i] {
                return 0;
            }
        }
        id
    })
    .sum::<u32>();
    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let res = input.lines()
    .map(extract_color_numbers_and_id)
    .map(|extracted| {
        let (_, color_numbers) = extracted;
        color_numbers[0] * color_numbers[1] * color_numbers[2]
    })
    .sum::<u32>();
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
