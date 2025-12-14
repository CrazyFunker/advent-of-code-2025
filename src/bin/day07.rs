use advent_of_code_2025::read_input;
use std::time::Instant;

fn main() {
    println!("--- Day 7 ---");
    match read_input(7) {
        Ok(input) => {
            let start_part1 = Instant::now();
            let part1_result = part1(&input);
            let part1_duration = start_part1.elapsed();
            println!("Part 1: {} (took {:?})", part1_result, part1_duration);

            let start_part2 = Instant::now();
            let part2_result = part2(&input);
            let part2_duration = start_part2.elapsed();
            println!("Part 2: {} (took {:?})", part2_result, part2_duration);
        }
        Err(e) => {
            eprintln!("Error reading input: {}", e);
        }
    }
}

fn part1(input: &str) -> usize {
    let line_len = input.lines().next().unwrap().len();
    let mut tachyon_tracker: Vec<bool> = vec![false; line_len];
    let mut split_count = 0;

    // find position of 'S' in the first line
    for (i, c) in input.lines().next().unwrap().chars().enumerate() {
        if c == 'S' {
            tachyon_tracker[i] = true;
        }
    }

    // iterate over each line (skipping first) and update tachyon_tracker
    for line in input.lines().skip(1) {
        for (i, c) in line.chars().enumerate() {
            if c == '^' && tachyon_tracker[i] {
                tachyon_tracker[i] = false;
                tachyon_tracker[i.saturating_sub(1)] = true;
                if i < line_len - 1 {
                    tachyon_tracker[i + 1] = true;
                }
                split_count += 1;
            }
        }
    }

    split_count
}

fn part2(input: &str) -> u64 {
    let line_len = input.lines().next().unwrap().len();
    let mut tachyon_tracker: Vec<u64> = vec![0; line_len];

    // find position of 'S' in the first line
    for (i, c) in input.lines().next().unwrap().chars().enumerate() {
        if c == 'S' {
            tachyon_tracker[i] = 1;
        }
    }

    // iterate over each line (skipping first) and update tachyon_tracker
    for line in input.lines().skip(1) {
        for (i, c) in line.chars().enumerate() {
            if c == '^' && tachyon_tracker[i] > 0 {
                let tachyons = tachyon_tracker[i];

                tachyon_tracker[i] = 0;
                tachyon_tracker[i.saturating_sub(1)] += tachyons;
                tachyon_tracker[i + 1] += tachyons;
            }
        }
    }

    tachyon_tracker.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2_5() {
        let input = "...S...\n......\n...^...\n.......\n..^.^..\n.......\n.^...^."; // Provide a test input
        let result = part2(input);
        assert_eq!(result, 6); // Provide the expected result
    }

    #[test]
    fn test_part2_4() {
        let input = "...S...\n......\n...^...\n.......\n..^.^..\n.......\n.^.^.^."; // Provide a test input
        let result = part2(input);
        assert_eq!(result, 8); // Provide the expected result
    }

    #[test]
    fn test_part2_3() {
        let input = "..S..\n.....\n..^..\n.....\n.^.^."; // Provide a test input
        let result = part2(input);
        assert_eq!(result, 4); // Provide the expected result
    }

    #[test]
    fn test_part2_2() {
        let input = "..S..\n.....\n..^..\n....."; // Provide a test input
        let result = part2(input);
        assert_eq!(result, 2); // Provide the expected result
    }

    #[test]
    fn test_part2_1() {
        let input = "..S..\n.....\n.^.^.\n....."; // Provide a test input
        let result = part2(input);
        assert_eq!(result, 1); // Provide the expected result
    }
}
