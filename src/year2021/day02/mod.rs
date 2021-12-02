use crate::log;
use std::{fs, ops::Index};

fn basic_navigation(contents: &str) -> i32 {
    let [position, depth]: [i32; 2] = contents
        .lines()
        .map(|s| {
            let values: Vec<&str> = s.split(" ").collect();
            let direction = values.index(0);
            let size = values.index(1).parse::<i32>().unwrap();

            return match direction.to_owned() {
                "forward" => [size, 0],
                "down" => [0, size],
                "up" => [0, -size],
                &_ => [0, 0],
            };
        })
        .reduce(|a, b| [a[0] + b[0], a[1] + b[1]])
        .unwrap();

    return position * depth;
}

fn aim_navigation(contents: &str) -> i32 {
    let mut position = 0;
    let mut depth = 0;
    let mut aim = 0;

    let lines = contents.lines().collect::<Vec<&str>>();

    for line in lines {
        let values: Vec<&str> = line.split(" ").collect();
        let direction = values.index(0).to_owned();
        let size = values.index(1).parse::<i32>().unwrap();

        if direction == "down" {
            aim = aim + size;
        } else if direction == "up" {
            aim = aim - size;
        } else if direction == "forward" {
            position = position + size;
            depth = depth + aim * size;
        }
    }

    return position * depth;
}

pub fn main() {
    println!("2021/02");
    let contents = fs::read_to_string("src/year2021/day02/input.txt").expect("File doesn't exist");
    log("Basic navigation", &|| basic_navigation(&contents));
    log("Aim navigation", &|| aim_navigation(&contents));
}
