pub fn part1(input: &str) -> usize {
    let lines: Vec<Vec<&str>> = input.lines().map(|line| line.split_whitespace().collect()).collect();
    let mut grand_total: usize = 0;

    let signs = lines.last().unwrap();

    for i in 0..signs.len() {
        println!("Processing column {}", i);
        let sign = signs[i];
        print!("Sign for {}th column: {}\n", i, sign);
        let mut total: usize = 0;

        for j in 0..lines.len() - 1 {
            let num: usize = lines[j][i].parse().unwrap();
            println!("Number at row {}, column {}: {}", j, i, num);
            match sign {
                "+" => total += num,
                "*" => {
                    if total == 0 {
                        total = 1;
                    }
                    total *= num;
                }
                _ => panic!("Unknown sign"),
            }
        }

        println!("Total for {}th column: {}", i, total);
        grand_total += total;
    }

    grand_total as usize
}

pub fn part2(input: &str) -> usize {
    let mut grand_total: usize = 0;

    let raw_lines: Vec<&str> = input.lines().collect();
    let num_cols = raw_lines.iter().map(|line| line.len()).max().unwrap_or(0);
    
    // Check which columns are all whitespace
    let mut is_separator: Vec<bool> = vec![true; num_cols];
    for line in &raw_lines {
        for (i, ch) in line.chars().enumerate() {
            if !ch.is_whitespace() {
                is_separator[i] = false;
            }
        }
    }
    
    // Split based on separator columns
    let lines: Vec<Vec<&str>> = raw_lines.iter().map(|line| {
        let mut words = Vec::new();
        let mut start = 0;
        let chars: Vec<char> = line.chars().collect();
        
        for i in 0..chars.len() {
            if is_separator.get(i).copied().unwrap_or(false) {
                if start < i {
                    words.push(&line[start..i]);
                }
                start = i + 1;
            }
        }
        if start < line.len() {
            words.push(&line[start..]);
        }
        words
    }).collect();

    let signs = lines.last().unwrap();

    for i in (0..signs.len()).rev() {
        println!("Processing column {}", i);
        let sign = signs[i];
        print!("Sign for {}th column: \"{}\"\n", i, sign);
        let mut total: usize = 0;

        let problem_raw_inputs: Vec<&str> = lines.iter().take(lines.len() - 1).map(|line| line[i]).collect();

        for k in (0..problem_raw_inputs[0].len()).rev() {
            let mut num_str = String::new();
            for l in 0..problem_raw_inputs.len() {
                let ch = problem_raw_inputs[l].chars().nth(k).unwrap();
                if !ch.is_whitespace() {
                    num_str.push(ch);
                }
            }
            if !num_str.is_empty() {
                let num: usize = num_str.parse().unwrap();
                println!("Number formed at digit position {}: {}", k, num);
                match sign.trim() {
                    "+" => total += num,
                    "*" => {
                        if total == 0 {
                            total = 1;
                        }
                        total *= num;
                    }
                    _ => panic!("Unknown sign"),
                }

            }

        }

        println!("Total for {}th column: {}", i, total);
        grand_total += total;
    }


    grand_total
}

mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("test.txt");
        let result = part1(input);

        assert_eq!(result, 4277556);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("test.txt");
        let result = part2(input);

        assert_eq!(result, 3263827);
    }
}