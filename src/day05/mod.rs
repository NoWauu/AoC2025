
pub fn part1(input: &str) -> usize {
    let lines = input.lines().collect::<Vec<&str>>();
    // Ranges are all lines before the blank line
    let ranges = lines
        .iter()
        .take_while(|&&line| !line.is_empty())
        .collect::<Vec<&&str>>();

    let ids = lines.iter().skip(ranges.len() + 1).collect::<Vec<&&str>>();

    let mut result = 0;

    for id in ids {
        let mut valid = false;
        for range in &ranges {
            let parts = range.split('-').collect::<Vec<&str>>();
            let start: usize = parts[0].parse().unwrap();
            let end: usize = parts[1].parse().unwrap();

            let i: isize = id.parse().unwrap();
            let is_in_range: bool = i as usize >= start && i as usize <= end;
            if is_in_range {
                valid = true;
                break;
            } 

        }
        if valid {
            result += 1;
        }
    }

    result
}
pub fn part2(input: &str) -> usize {
    let lines = input.lines().collect::<Vec<&str>>();
    
    // Parse all ranges
    let mut ranges: Vec<(isize, isize)> = lines
        .iter()
        .take_while(|&&line| !line.is_empty())
        .map(|line| {
            let parts: Vec<&str> = line.split('-').collect();
            let start: isize = parts[0].parse().unwrap();
            let end: isize = parts[1].parse().unwrap();
            (start, end)
        })
        .collect();
    
    // Sort ranges by start position
    ranges.sort_unstable();
    
    // Merge overlapping ranges
    let mut merged: Vec<(isize, isize)> = Vec::new();
    
    for (start, end) in ranges {
        if let Some(last) = merged.last_mut() {
            if start <= last.1 + 1 {
                // Overlapping or adjacent, merge them
                last.1 = last.1.max(end);
            } else {
                // Non-overlapping, add new range
                merged.push((start, end));
            }
        } else {
            merged.push((start, end));
        }
    }
    
    // Calculate total count
    merged.iter().map(|(start, end)| (end - start + 1) as usize).sum()
}

mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("test.txt");
        let result = part1(input);
        assert_eq!(result, 3); 
    }

    #[test]
    fn test_part2() {
        let input = include_str!("test.txt");
        let result = part2(input);
        assert_eq!(result, 14); 
    }
}