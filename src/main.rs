use std::fs::read_to_string;
use std::time::Instant;

mod _2020;
mod _2022;

pub fn main() {
    print_head();
    run(2020, 1, vec![_2020::_01::part1, _2020::_01::part2]);
    run(2022, 1, vec![_2022::_01::part1, _2022::_01::part2]);
    run(2022, 2, vec![_2022::_02::part1, _2022::_02::part2]);
    run(2022, 3, vec![_2022::_03::part1, _2022::_03::part2]);
    run(2022, 4, vec![_2022::_04::part1, _2022::_04::part2]);
}

fn print_head() {
    println!(
        "{: <7}  {: <20}  {: <10}  {: <10}",
        "Date", "Input type", "Time", "Result"
    );

    println!(
        "{: <7}  {: <20}  {: <10}  {: <10}",
        "----", "----------", "----", "------"
    );
}

fn run(year: usize, day: usize, fns: Vec<fn(String) -> String>) {
    for input in ["example", "input"] {
        for (index, f) in fns.iter().enumerate() {
            let start = Instant::now();
            let result = f(read(year, day, input));
            let time = format!("{:?}", start.elapsed());
            let name = format!("{} (part {})", input, index + 1);

            println!(
                "{}/{:0>2}  {: <20}  {: <10}  {: <10}",
                year, day, name, time, result
            );
        }
    }
}

fn read(year: usize, day: usize, name: &str) -> String {
    let file_name = format!("./src/_{}/_{:0>2}/{}.txt", year, day, name);
    read_to_string(file_name.as_str()).expect(file_name.as_str())
}
