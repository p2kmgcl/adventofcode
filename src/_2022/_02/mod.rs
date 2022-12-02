const SCORE_LOSE: u32 = 0;
const SCORE_DRAW: u32 = 3;
const SCORE_WIN: u32 = 6;

const SCORE_ROCK: u32 = 1;
const SCORE_PAPER: u32 = 2;
const SCORE_SCISSORS: u32 = 3;

pub fn part1(input: String) -> String {
    let mut score: u32 = 0;

    for line in input.lines() {
        score = score + get_line_score_part1(line);
    }

    score.to_string()
}

pub fn part2(input: String) -> String {
    let mut score: u32 = 0;

    for line in input.lines() {
        score = score + get_line_score_part2(line);
    }

    score.to_string()
}

fn get_line_score_part1(line: &str) -> u32 {
    match line.chars().nth(0).unwrap() {
        'A' => match line.chars().nth(2).unwrap() {
            'X' => SCORE_ROCK + SCORE_DRAW,
            'Y' => SCORE_PAPER + SCORE_WIN,
            'Z' => SCORE_SCISSORS + SCORE_LOSE,
            _ => 0,
        },
        'B' => match line.chars().nth(2).unwrap() {
            'X' => SCORE_ROCK + SCORE_LOSE,
            'Y' => SCORE_PAPER + SCORE_DRAW,
            'Z' => SCORE_SCISSORS + SCORE_WIN,
            _ => 0,
        },
        'C' => match line.chars().nth(2).unwrap() {
            'X' => SCORE_ROCK + SCORE_WIN,
            'Y' => SCORE_PAPER + SCORE_LOSE,
            'Z' => SCORE_SCISSORS + SCORE_DRAW,
            _ => 0,
        },
        _ => 0,
    }
}

fn get_line_score_part2(line: &str) -> u32 {
    let result_score: u32 = match line.chars().nth(2).unwrap() {
        'X' => SCORE_LOSE,
        'Y' => SCORE_DRAW,
        'Z' => SCORE_WIN,
        _ => 0,
    };

    let participation_score = match line.chars().nth(0).unwrap() {
        'A' => match line.chars().nth(2).unwrap() {
            'X' => SCORE_SCISSORS,
            'Y' => SCORE_ROCK,
            'Z' => SCORE_PAPER,
            _ => 0,
        },
        'B' => match line.chars().nth(2).unwrap() {
            'X' => SCORE_ROCK,
            'Y' => SCORE_PAPER,
            'Z' => SCORE_SCISSORS,
            _ => 0,
        },
        'C' => match line.chars().nth(2).unwrap() {
            'X' => SCORE_PAPER,
            'Y' => SCORE_SCISSORS,
            'Z' => SCORE_ROCK,
            _ => 0,
        },
        _ => 0,
    };

    result_score + participation_score
}
