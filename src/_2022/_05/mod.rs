use itertools::Itertools;
use regex::Regex;
use std::cmp::max;
use std::collections::VecDeque;

enum ParseStep {
    Stacking,
    Moving,
}

pub fn part1(input: String) -> String {
    simulate_movement(input, |stacks, movements, from_index, to_index| {
        for _index in 0..movements {
            let c = {
                let from_stack = &mut stacks[from_index - 1];
                from_stack.pop_front().unwrap()
            };

            let to_stack = &mut stacks[to_index - 1];
            to_stack.push_front(c);
        }
    })
}

pub fn part2(input: String) -> String {
    simulate_movement(input, |stacks, movements, from_index, to_index| {
        let mut crates: Vec<char> = vec![];

        for _index in 0..movements {
            let from_stack = &mut stacks[from_index - 1];
            crates.push(from_stack.pop_front().unwrap());
        }

        for c in crates.iter().rev() {
            let to_stack = &mut stacks[to_index - 1];
            to_stack.push_front(*c);
        }
    })
}

fn simulate_movement(
    input: String,
    move_crates: fn(
        stacks: &mut Vec<VecDeque<char>>,
        movements: usize,
        from_index: usize,
        to_index: usize,
    ) -> (),
) -> String {
    let mut parse_step = ParseStep::Stacking;
    let mut stacks: Vec<VecDeque<char>> = vec![];

    for line in input.lines() {
        match parse_step {
            ParseStep::Stacking => {
                if !line.contains("[") {
                    parse_step = ParseStep::Moving;
                    continue;
                }

                for (index, c) in line.char_indices() {
                    if c.is_alphabetic() {
                        let stack_index = index / 4;
                        stacks.resize_with(max(stacks.len(), stack_index + 1), || VecDeque::new());
                        let stack = &mut stacks[stack_index];
                        stack.push_back(c);
                    }
                }
            }
            ParseStep::Moving => {
                let r = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();

                if !r.is_match(line) {
                    continue;
                }

                let (movements, from_index, to_index) = r
                    .captures(line)
                    .unwrap()
                    .iter()
                    .skip(1)
                    .map(|v| v.unwrap().as_str().parse::<usize>().unwrap())
                    .next_tuple()
                    .unwrap();

                move_crates(&mut stacks, movements, from_index, to_index);
            }
        }
    }

    stacks
        .into_iter()
        .filter(|stack| stack.len() > 0)
        .map(|stack| *stack.iter().next().unwrap())
        .join("")
}
