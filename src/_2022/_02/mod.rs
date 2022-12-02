const SCORE_LOSE: u32 = 0;
const SCORE_DRAW: u32 = 3;
const SCORE_WIN: u32 = 6;

const SCORE_ROCK: u32 = 1;
const SCORE_PAPER: u32 = 2;
const SCORE_SCISSORS: u32 = 3;

pub fn part1(input: String) -> String {
    input
        .lines()
        .map(get_line_score_part1)
        .sum::<u32>()
        .to_string()
}

pub fn part2(input: String) -> String {
    input
        .lines()
        .map(get_line_score_part2)
        .sum::<u32>()
        .to_string()
}

fn get_line_score_part1(line: &str) -> u32 {
    match line {
        "A X" => SCORE_ROCK + SCORE_DRAW,
        "A Y" => SCORE_PAPER + SCORE_WIN,
        "A Z" => SCORE_SCISSORS + SCORE_LOSE,
        "B X" => SCORE_ROCK + SCORE_LOSE,
        "B Y" => SCORE_PAPER + SCORE_DRAW,
        "B Z" => SCORE_SCISSORS + SCORE_WIN,
        "C X" => SCORE_ROCK + SCORE_WIN,
        "C Y" => SCORE_PAPER + SCORE_LOSE,
        "C Z" => SCORE_SCISSORS + SCORE_DRAW,
        _ => 0,
    }
}

fn get_line_score_part2(line: &str) -> u32 {
    match line {
        "A X" => SCORE_SCISSORS + SCORE_LOSE,
        "A Y" => SCORE_ROCK + SCORE_DRAW,
        "A Z" => SCORE_PAPER + SCORE_WIN,
        "B X" => SCORE_ROCK + SCORE_LOSE,
        "B Y" => SCORE_PAPER + SCORE_DRAW,
        "B Z" => SCORE_SCISSORS + SCORE_WIN,
        "C X" => SCORE_PAPER + SCORE_LOSE,
        "C Y" => SCORE_SCISSORS + SCORE_DRAW,
        "C Z" => SCORE_ROCK + SCORE_WIN,
        _ => 0,
    }
}
