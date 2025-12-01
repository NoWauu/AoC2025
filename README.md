# Advent of Code 2025 - Rust

This repository contains solutions for Advent of Code 2025 in Rust.

## Structure

Each day is a separate module in the `src` directory.
- `src/dayXX/mod.rs`: Contains the solution code (`part1` and `part2`).
- `src/dayXX/input.txt`: Contains the puzzle input.

## Running

To run the solutions:

```bash
cargo run
```

## Adding a new day

1. Create a new directory `src/dayXX`.
2. Create `src/dayXX/input.txt` and paste your input.
3. Create `src/dayXX/mod.rs` with the following template:

```rust
pub fn part1(input: &str) -> String {
    // TODO: Solve Part 1
    "Not implemented".to_string()
}

pub fn part2(input: &str) -> String {
    // TODO: Solve Part 2
    "Not implemented".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "";
        assert_eq!(part1(input), "Not implemented");
    }

    #[test]
    fn test_part2() {
        let input = "";
        assert_eq!(part2(input), "Not implemented");
    }
}
```

4. Register the module in `src/main.rs`:

```rust
mod dayXX;

fn main() {
    // ... previous days ...

    // Day XX
    let dayXX_input = include_str!("dayXX/input.txt");
    println!("--- Day XX ---");
    println!("Part 1: {}", dayXX::part1(dayXX_input));
    println!("Part 2: {}", dayXX::part2(dayXX_input));
}
```
