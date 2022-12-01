pub fn part1(input: String) -> String {
    let values: Vec<i32> = input.lines().map(|line| line.parse().unwrap()).collect();

    for x in &values {
        if *x >= 2020 {
            continue;
        }

        for y in &values {
            if x + y == 2020 {
                return format!("{}", x * y);
            }
        }
    }

    return "0".to_string();
}

pub fn part2(input: String) -> String {
    let values: Vec<i32> = input.lines().map(|line| line.parse().unwrap()).collect();

    for x in &values {
        if *x >= 2020 {
            continue;
        }

        for y in &values {
            if x + y >= 2020 {
                continue;
            }

            for z in &values {
                if x + y + z == 2020 {
                    return format!("{}", x * y * z);
                }
            }
        }
    }

    return "0".to_string();
}
