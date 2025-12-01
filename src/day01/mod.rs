pub fn part1(input: &str) -> i32 {
    let input_vec = input.lines().collect::<Vec<&str>>();
    let mut current_pos = 50;
    let mut password = 0;

    for i in input_vec {
        let dir = &i[0..1];
        let val = &i[1..].trim().parse::<i32>().unwrap();

        match dir {
            "L" => current_pos -= val,
            "R" => current_pos += val,
            _ => {}
        }
        current_pos = current_pos.rem_euclid(100);
        if current_pos == 0 {
            password += 1;
        }
    }
    password
}

pub fn part2(input: &str) -> i32 {
    let mut current_pos = 50;
    let mut password = 0;

    for line in input.lines() {
        let val = line[1..].trim().parse::<i32>().unwrap();
        let prev = current_pos;

        if line.starts_with('R') {
            current_pos += val;
            password += current_pos.div_euclid(100) - prev.div_euclid(100);
        } else {
            current_pos -= val;
            password += (prev - 1).div_euclid(100) - (current_pos - 1).div_euclid(100);
        }
        current_pos = current_pos.rem_euclid(100);
    }
    password
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("test.txt");
        assert_eq!(part1(input), 3);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("test.txt");
        assert_eq!(part2(input), 6);
    }
}
