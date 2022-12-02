#!/bin/bash

year=$1
day=$2

if [ -f "src/_${year}/_${day}/mod.rs" ]; then
    echo "${year}/${day}" already exists
    exit 1
fi

if [ ! -f "src/_${year}/mod.rs" ]; then
    mkdir "src/_${year}"
    echo "" > "src/_${year}/mod.rs";
    line_number=$(grep -n "mod _" "src/main.rs" | tail -1 | cut -d':' -f1)
    sed -i "${line_number} a mod _${year};" "src/main.rs"
fi

mkdir "src/_${year}/_${day}"
touch "src/_${year}/_${day}/example.txt"
touch "src/_${year}/_${day}/input.txt"

echo 'pub fn part1(input: String) -> String {
    "ToDo".to_string()
}

pub fn part2(input: String) -> String {
    "ToDo".to_string()
}
' > "src/_${year}/_${day}/mod.rs"

sed -i "1 i pub mod _${day};" "src/_${year}/mod.rs"
line_number=$(grep -n "    run(" "src/main.rs" | tail -1 | cut -d':' -f1)
sed -i "${line_number} a \ \ \ \ run(${year}, $(echo $day | sed 's/^0*//'), vec![_${year}::_${day}::part1, _${year}::_${day}::part2]);" "src/main.rs"
