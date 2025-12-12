pub fn part1(input: &str) -> usize {
    let lines: Vec<&str> = input.lines().collect();
    let mut result = 0;

    for i in 0..lines.len() {
        let line = lines[i];
        for j in 0..line.len() {
            let count = get_neighbors_count(&lines, i, j);
            if count < 4 {
                result += 1;
            }
        }
    }
    result
}

fn get_neighbors_count(lines: &Vec<&str>, i: usize, j: usize) -> usize {
    let mut count = 0;
    let line = lines[i];
    let bytes = line.as_bytes();
    if bytes[j] == b'@' {
        for di in -1..=1 {
            for dj in -1..=1 {
                if di == 0 && dj == 0 {
                    continue;
                }
                let ni = i as isize + di;
                let nj = j as isize + dj;
                if ni >= 0 && ni < lines.len() as isize && nj >= 0 && nj < line.len() as isize {
                    if lines[ni as usize].as_bytes()[nj as usize] == b'@' {
                        count += 1;
                    }
                }
            }
        }
    } else {
        count = usize::MAX;
    }
    count
}

pub fn part2(input: &str) -> usize {
    let mut current_input: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    let mut result = 0;
    let mut keep_going = true;

    while keep_going {
        let mut current_count = 0;
        let mut new_input = current_input.clone();
        
        for i in 0..current_input.len() {
            let line = &current_input[i];
            for j in 0..line.len() {
                let count = count_neighbors(&current_input, i, j);
                if count < 4 {
                    current_count += 1;
                    let mut chars: Vec<char> = new_input[i].chars().collect();
                    chars[j] = '.';
                    new_input[i] = chars.into_iter().collect();
                }
            }
        }
        
        result += current_count;
        current_input = new_input;
        
        if current_count == 0 {
            keep_going = false;
        }
    }
    result
}

fn count_neighbors(lines: &Vec<String>, i: usize, j: usize) -> usize {
    let mut count = 0;
    let line = &lines[i];
    let bytes = line.as_bytes();
    if bytes[j] == b'@' {
        for di in -1..=1 {
            for dj in -1..=1 {
                if di == 0 && dj == 0 {
                    continue;
                }
                let ni = i as isize + di;
                let nj = j as isize + dj;
                if ni >= 0 && ni < lines.len() as isize && nj >= 0 && nj < line.len() as isize {
                    if lines[ni as usize].as_bytes()[nj as usize] == b'@' {
                        count += 1;
                    }
                }
            }
        }
    } else {
        count = usize::MAX;
    }
    count
}

mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("test.txt");
        assert_eq!(part1(input), 13);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("test.txt");
        assert_eq!(part2(input), 43);
    }
}