use std::{fs, ops::Index};

fn simple_increase(values: &Vec<i32>) -> u32 {
    let mut count: u32 = 0;

    for i in 1..values.len() {
        if values.index(i - 1) < values.index(i) {
            count = count + 1;
        }
    }

    return count;
}

fn window_increase(values: &Vec<i32>, window_size: usize) -> usize {
    let mut count = 0;

    for i in 0..(values.len() - window_size) {
        let current: i32 = values[i..(i + window_size)].iter().sum();
        let next: i32 = values[(i + 1)..(i + window_size + 1)].iter().sum();

        if current < next {
            count = count + 1;
        }
    }

    return count;
}

pub fn main() {
    println!("2021/01");

    let values = fs::read_to_string("src/year2021/day01/input.txt")
        .expect("File doesn't exist")
        .lines()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    println!("Simple increase: {}", simple_increase(&values));
    println!("Window increase (3): {}", window_increase(&values, 3));
    println!("Window increase (1): {}", window_increase(&values, 1));
}
