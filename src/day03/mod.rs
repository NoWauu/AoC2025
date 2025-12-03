
pub fn part_1(input: &str) -> isize {
    let lines: Vec<&str> = input.lines().collect();

    let mut total = 0;

    for line in &lines {
        let mut max = 0;
        for i in 0..line.len() {
            let digit = line.chars().nth(i).unwrap();
            for j in i+1..line.len() {
                if i == j {
                    continue;
                }

                let digit2 = line.chars().nth(j).unwrap();

                let concat = format!("{}{}", digit, digit2);

                if concat.parse::<isize>().unwrap() > max {
                    max = concat.parse::<isize>().unwrap();
                }
            }
        }

        total += max;
    }
    total

} 


pub fn max_k_concat(line: &str, k: usize) -> isize {
    let digits: Vec<u8> = line.bytes().map(|b| b - b'0').collect();
    let n = digits.len();

    let mut result: Vec<u8> = Vec::with_capacity(k);
    let mut start = 0;

    for remaining in (0..k).rev() {
        // We need (remaining + 1) digits including the one we pick now.
        // So the furthest we can search is:
        let end = n - remaining;

        // Find the max digit in digits[start..end]
        let mut max_digit = 0;
        let mut max_index = start;

        for i in start..end {
            if digits[i] > max_digit {
                max_digit = digits[i];
                max_index = i;
                if max_digit == 9 {
                    break; // cannot do better
                }
            }
        }

        result.push(max_digit);
        start = max_index + 1;
    }

    // Convert the 12-digit (or k-digit) number into an isize
    let mut value: isize = 0;
    for d in result {
        value = value * 10 + d as isize;
    }
    value
}

pub fn part2(input: &str) -> isize {
    input.lines().map(|line| max_k_concat(line, 12)).sum()
}


mod tests {
    use super::*;

    #[test]
    pub fn part1_test() {
        let input = include_str!("test.txt");

        let res = part1(input);

        assert_eq!(res, 357);
    }

    #[test]
    pub fn part2_test() {
        let input = include_str!("test.txt");

        let res = part2(input);
        assert_eq!(res, 3121910778619);
    }
}