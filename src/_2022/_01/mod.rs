pub fn part1(input: String) -> String {
    max_calories(input, 1)
}

pub fn part2(input: String) -> String {
    max_calories(input, 3)
}

fn max_calories(input: String, elf_count: usize) -> String {
    let mut current_calories: u32 = 0;
    let mut max_calories_list: Vec<u32> = vec![0; elf_count];

    for line in input.lines() {
        if line.is_empty() {
            if max_calories_list[0] < current_calories {
                max_calories_list.push(current_calories);
                max_calories_list.sort();
                max_calories_list =
                    max_calories_list[max_calories_list.len() - elf_count..].to_vec();
            }

            current_calories = 0;
        } else {
            let calories: u32 = line.parse().expect("calories");
            current_calories = current_calories + calories;
        }
    }

    max_calories_list.iter().sum::<u32>().to_string()
}
