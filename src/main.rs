mod day01;

fn main() {
    // Day 01
    let day01_input = include_str!("day01/input.txt");
    println!("--- Day 01 ---");
    println!("Part 1: {}", day01::part1(day01_input));
    println!("Part 2: {}", day01::part2(day01_input));
}

