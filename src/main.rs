use std::fmt::Display;
use std::time::Instant;
pub mod year2020;
pub mod year2021;

pub fn log<T: Display>(name: &str, routine: &dyn Fn() -> T) {
    let mut durations: Vec<u128> = vec![];
    let mut result: T = routine();

    for _ in 0..10 {
        let now = Instant::now();
        result = routine();
        durations.push(now.elapsed().as_micros());
    }

    println!(
        "{}: {} (in {}μs)",
        name,
        result,
        durations.iter().sum::<u128>() / u128::try_from(durations.len()).unwrap()
    );
}

fn main() {
    year2020::main();
    year2021::main();
}
