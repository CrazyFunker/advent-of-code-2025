use advent_of_code_2025::read_input;
use std::time::Instant;

fn main() {
    println!("--- Day 2 ---");
    match read_input(2) {
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

fn part1(input: &str) -> u64 {
    // fetch the first line from the input, split by comma and store as a vector
    let first_line = input.lines().next().unwrap_or("");
    let items: Vec<&str> = first_line.split(',').collect();
    let mut sum_of_invalid_ids: u64 = 0;
    // iterate over the items
    for item in &items {
        // split item by hyphen
        let parts: Vec<&str> = item.split('-').collect();
        let start: u64 = parts[0].parse().unwrap_or(0);
        let end: u64 = parts[1].parse().unwrap_or(0);
        let mut invalid_ids: Vec<u64> = vec![];

        // iterate from start to end, end inclusive
        for number in start..=end {
            // calculate the number of digits in the number, without converting to string
            let mut n = number;
            let mut digit_count = 0;
            while n > 0 {
                n /= 10;
                digit_count += 1;
            }
            // check if digit_count is even
            if digit_count > 0 && digit_count % 2 == 0 {
                // extract the first half and second half of the number
                // e.g. 1234 -> first_half = 12, second_half = 34
                // 4 * 1
                let mut decimal_multiplier: u64 = 10_u64.pow(digit_count / 2);
                let mut first_half: u64 = number / decimal_multiplier;
                let mut second_half: u64 = number - first_half * decimal_multiplier;
                
                if first_half == second_half {
                    println!("Invalid ID: {}", number);
                    invalid_ids.push(number);
                }
            }
        }

        sum_of_invalid_ids = sum_of_invalid_ids + invalid_ids.iter().sum::<u64>();
    }

    sum_of_invalid_ids
}

// function extracting first part and last part of a number

fn part2(input: &str) -> usize {
    input.chars().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = ""; // Provide a test input
        let result = part1(input);
        assert_eq!(result, 0); // Provide the expected result
    }

    #[test]
    fn test_part2() {
        let input = ""; // Provide a test input
        let result = part2(input);
        assert_eq!(result, 0); // Provide the expected result
    }
}
