use itertools::Itertools;

pub fn part1(input: String) -> String {
    count_conflicts(input, |a, b| {
        (a.0 >= b.0 && a.1 <= b.1) || (b.0 >= a.0 && b.1 <= a.1)
    })
    .to_string()
}

pub fn part2(input: String) -> String {
    count_conflicts(input, |a, b| {
        (a.0 >= b.0 && a.0 <= b.1)
            || (a.1 >= b.0 && a.1 <= b.1)
            || (b.0 >= a.0 && b.0 <= a.1)
            || (b.1 >= a.0 && b.1 <= a.1)
    })
    .to_string()
}

fn count_conflicts(input: String, has_conflict: fn((u32, u32), (u32, u32)) -> bool) -> usize {
    input
        .lines()
        .filter(|line| {
            let (a, b) = line
                .split(",")
                .map(|chunk| {
                    chunk
                        .split("-")
                        .map(|num| num.parse::<u32>().unwrap())
                        .next_tuple::<(u32, u32)>()
                        .unwrap()
                })
                .next_tuple()
                .unwrap();

            has_conflict(a, b)
        })
        .count()
}
