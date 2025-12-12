use advent_of_code_2025::read_input;
use std::time::Instant;

fn main() {
    println!("--- Day 3 ---");
    match read_input(3) {
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
    // iterate over each line
    let mut sum: usize = 0;
    for line in input.lines() {
        sum += find_largest_joltage(line);
    }

    sum
}

fn find_largest_joltage(line: &str) -> usize {
    let c: Vec<u32> = line.chars()
        .filter_map(|c| c.to_digit(10))
        .collect();

    let mut pointer_1 = 0;
    let mut pointer_2 = 1;
    let mut max_joltage = 0;

    while pointer_1 < c.len() - 1 {
        while pointer_2 < c.len() {
            let joltage = c[pointer_1]*10 + c[pointer_2];
            if joltage > max_joltage {
                max_joltage = joltage;
            }
            pointer_2 += 1;
        }
        pointer_1 += 1;
        pointer_2 = pointer_1 + 1;
    }

    max_joltage as usize
}

fn part2(input: &str) -> usize {
    input.chars().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "12";
        assert_eq!(find_largest_joltage(input), 12); // Replace 0 with the expected result
    }

    #[test]
    fn test_part1_1() {
        let input = "12345";
        assert_eq!(find_largest_joltage(input), 45); // Replace 0 with the expected result
    }

}
