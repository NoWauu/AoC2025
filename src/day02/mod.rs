pub fn part1(input: &str) -> isize {
    let ranges: Vec<&str> = input.split(",").collect();
    let mut sum_invalid = 0;

    for range in ranges {
        let parts: Vec<&str> = range.split('-').collect();
        let start: isize = parts[0].parse().unwrap();
        let end: isize = parts[1].parse().unwrap();

        for i in start..=end {
            let i_str = i.to_string();
            // split the string in half
            let half_size = i_str.len() / 2;
            let half1 = i_str[0..half_size].to_string();
            let half2 = i_str[half_size..].to_string();

            if half1 == half2 {
                sum_invalid += i;
            }
        }
    }

    sum_invalid
}

pub fn part2(input: &str) -> isize {
    let ranges: Vec<&str> = input.split(",").collect();
    let mut sum_invalid = 0;

    for range in ranges {
        let parts: Vec<&str> = range.split('-').collect();
        let start: isize = parts[0].parse().unwrap();
        let end: isize = parts[1].parse().unwrap();

        for i in start..=end {
            let i_str = i.to_string();
            let len = i_str.len();
            let mut found = false;

            for chunk_size in 1..=len / 2 {
                if len % chunk_size == 0 {
                    let k = len / chunk_size;
                    let chunk = &i_str[0..chunk_size];
                    let mut all_match = true;
                    for j in 1..k {
                        if &i_str[j * chunk_size..(j + 1) * chunk_size] != chunk {
                            all_match = false;
                            break;
                        }
                    }
                    if all_match {
                        found = true;
                        break;
                    }
                }
            }

            if found {
                sum_invalid += i;
            }
        }
    }
    sum_invalid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn part1_test() {
        let input = include_str!("test.txt");

        let res = part1(input);
        assert_eq!(res, 1227775554);
    }

    #[test]
    pub fn part2_test() {
        let input = include_str!("test.txt");

        let res = part2(input);
        assert_eq!(res, 4174379265);
    }
}