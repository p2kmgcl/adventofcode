use itertools::Itertools;

pub fn part1(input: String) -> String {
    input
        .lines()
        .filter(|line| {
            let (definition, value) = line.split(": ").next_tuple().unwrap();
            MinMaxRule::new(definition).check(value)
        })
        .count()
        .to_string()
}

pub fn part2(input: String) -> String {
    input
        .lines()
        .filter(|line| {
            let (definition, value) = line.split(": ").next_tuple().unwrap();
            PositionRule::new(definition).check(value)
        })
        .count()
        .to_string()
}

trait Rule {
    fn new(definition: &str) -> Self;
    fn check(&self, input: &str) -> bool;
}

struct MinMaxRule {
    min: usize,
    max: usize,
    letter: char,
}

impl Rule for MinMaxRule {
    fn new(definition: &str) -> MinMaxRule {
        let (values_str, char_str) = definition.split(" ").next_tuple().unwrap();
        let letter = char_str.chars().next().unwrap();

        let (min, max) = values_str
            .split("-")
            .map(|v| v.parse::<usize>().unwrap())
            .next_tuple()
            .unwrap();

        MinMaxRule { min, max, letter }
    }

    fn check(&self, input: &str) -> bool {
        let mut letter_count: usize = 0;

        for char in input.chars() {
            if char == self.letter {
                letter_count += 1;
            }
        }

        letter_count >= self.min && letter_count <= self.max
    }
}

struct PositionRule {
    first_position: usize,
    second_position: usize,
    letter: char,
}

impl Rule for PositionRule {
    fn new(definition: &str) -> PositionRule {
        let (values_str, char_str) = definition.split(" ").next_tuple().unwrap();
        let letter = char_str.chars().next().unwrap();

        let (first_position, second_position) = values_str
            .split("-")
            .map(|v| v.parse::<usize>().unwrap())
            .next_tuple()
            .unwrap();

        PositionRule {
            first_position,
            second_position,
            letter,
        }
    }

    fn check(&self, input: &str) -> bool {
        let chars = input.chars().collect_vec();
        let first_char = *chars.get(self.first_position - 1).unwrap();
        let second_char = *chars.get(self.second_position - 1).unwrap();

        (first_char == self.letter && second_char != self.letter)
            || (first_char != self.letter && second_char == self.letter)
    }
}
