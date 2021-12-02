use crate::log;
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

fn native_increase(values: &Vec<i32>, window_size: usize) -> usize {
    let mut count = 0;
    let mut chunks = values.windows(window_size);
    let mut previous: i32 = chunks.next().unwrap().iter().sum();

    for chunk in chunks {
        let value = chunk.iter().sum();
        count = if value > previous { count + 1 } else { count };
        previous = value;
    }

    return count;
}

pub fn main() {
    println!("2021/01");

    let values: Vec<i32> = fs::read_to_string("src/year2021/day01/input.txt")
        .expect("File doesn't exist")
        .lines()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    log("Simple increase", &|| simple_increase(&values));
    log("Window increase (3)", &|| window_increase(&values, 3));
    log("Native increase (3)", &|| native_increase(&values, 3));
}
