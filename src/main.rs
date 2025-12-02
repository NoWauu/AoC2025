mod day01;
mod day02;

fn main() {
    // Day 01
    let day01_input = include_str!("day01/input.txt");
    println!("--- Day 01 ---");
    println!("Part 1: {}", day01::part1(day01_input));
    println!("Part 2: {}", day01::part2(day01_input));

    // Day 02
    let day02_input = include_str!("day02/input.txt");
    println!("--- Day 02 ---");
    println!("Part 1: {}", day02::part1(day02_input));
    println!("Part 2: {}", day02::part2(day02_input));

}

