use itertools::Itertools;

pub fn part1(input: String) -> String {
    input
        .lines()
        .map(|line| {
            let (left, right) = line.split_at(line.len() / 2);
            let common_char = left
                .chars()
                .find(|left_char| right.chars().any(|right_char| left_char == &right_char));
            get_char_value(common_char.unwrap())
        })
        .sum::<u32>()
        .to_string()
}

pub fn part2(input: String) -> String {
    input
        .lines()
        .tuples()
        .map(|(a, b, c)| {
            let common_char = a.chars().find(|a_char| {
                b.chars()
                    .any(|b_char| a_char == &b_char && c.chars().any(|c_char| a_char == &c_char))
            });

            get_char_value(common_char.unwrap())
        })
        .sum::<u32>()
        .to_string()
}

const BASE_LOWERCASE_COUNT: u32 = 96;
const BASE_UPPERCASE_COUNT: u32 = 38;

fn get_char_value(c: char) -> u32 {
    (c as u32)
        - if c.is_lowercase() {
            BASE_LOWERCASE_COUNT
        } else {
            BASE_UPPERCASE_COUNT
        }
}
