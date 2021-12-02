use crate::log;
use std::fs;

fn sum_two_values(values: &Vec<i32>) -> i32 {
    for x in values {
        if x >= &2020 {
            continue;
        }

        for y in values {
            if x + y == 2020 {
                return x * y;
            }
        }
    }

    return 0;
}

fn sum_three_values(values: &Vec<i32>) -> i32 {
    for x in values {
        if *x >= 2020 {
            continue;
        }

        for y in values {
            if x + y >= 2020 {
                continue;
            }

            for z in values {
                if x + y + z == 2020 {
                    return x * y * z;
                }
            }
        }
    }

    return 0;
}

pub fn main() {
    println!("2020/01");

    let values: Vec<i32> = fs::read_to_string("src/year2020/day01/input.txt")
        .expect("File doesn't exist")
        .lines()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    log("Sum two values", &|| sum_two_values(&values));
    log("Sum three values", &|| sum_three_values(&values));
}
