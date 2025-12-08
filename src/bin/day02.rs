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

fn part1(input: &str) -> usize {
    // fetch the first line from the input, split by comma and store as a vector
    let first_line = input.lines().next().unwrap_or("");
    let items: Vec<&str> = first_line.split(',').collect();

    // iterate over the items
    for item in &items {
        // split item by hyphen
        let parts: Vec<&str> = item.split('-').collect();
        let start: i32 = parts[0].parse().unwrap_or(0);
        let end: i32 = parts[1].parse().unwrap_or(0);
        let invalid_ids: Vec<i32> = vec![];

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
                let mut second_half = number / (10_i32.pow((digit_count) / 2));
                let mut first_half = number - second_half;
                println!(
                    "Number: {}, Digit Count: {}, First half: {}, Second half: {}",
                    number, digit_count, first_half, second_half
                );
            }
        }
    }

    items.len()
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
