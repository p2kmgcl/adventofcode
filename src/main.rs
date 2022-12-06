use std::fs::read_to_string;
use std::time::Instant;

mod _2020;
mod _2022;

pub fn main() {
    run(2020, 1, vec![_2020::_01::part1, _2020::_01::part2]);
    run(2022, 1, vec![_2022::_01::part1, _2022::_01::part2]);
    run(2022, 2, vec![_2022::_02::part1, _2022::_02::part2]);
    run(2022, 3, vec![_2022::_03::part1, _2022::_03::part2]);
    run(2022, 4, vec![_2022::_04::part1, _2022::_04::part2]);
    run(2022, 5, vec![_2022::_05::part1, _2022::_05::part2]);
    run(2022, 6, vec![_2022::_06::part1, _2022::_06::part2]);
}

fn run(year: usize, day: usize, fns: Vec<fn(String) -> String>) {
    println!("{}/{:0>2}\n-------", year, day);

    for (index, f) in fns.iter().enumerate() {
        for input_name in ["example", "input"] {
            let start = Instant::now();
            let result = f(read(year, day, input_name));
            let mut num: u128 = start.elapsed().as_secs() as u128;
            let mut unit = "s";

            if num <= 1 {
                num = start.elapsed().as_millis();
                unit = "ms";
            }

            if num <= 1 {
                num = start.elapsed().as_micros();
                unit = "μs";
            }

            let name = format!("Part {} ({: <7} in {}{})", index + 1, input_name, num, unit);

            println!("{: <28}:  {}", name, result);
        }
    }

    println!("");
}

fn read(year: usize, day: usize, name: &str) -> String {
    let file_name = format!("./src/_{}/_{:0>2}/{}.txt", year, day, name);
    read_to_string(file_name.as_str()).expect(file_name.as_str())
}
