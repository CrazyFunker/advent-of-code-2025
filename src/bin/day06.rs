use advent_of_code_2025::read_input;
use std::time::Instant;

fn main() {
    println!("--- Day 6 ---");
    match read_input(6) {
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
    let mut numbers: Vec<Vec<u64>> = vec![];
    let mut operations: Vec<&str> = vec![];
    let mut final_sum: u64 = 0;

    // iterate through each line
    for raw in input.lines() {
        // access first character to determine if it's an operation or number
        let first_char = raw.chars().next().unwrap();

        if first_char == '+' || first_char == '*' {
            // process as operation
            operations = raw.split_whitespace().collect();
        } else {
            // process as numbers
            numbers.push(raw.split_whitespace()
                .map(|s| s.parse::<u64>().unwrap())
                .collect());
        }
        // process line
    }

    let number_of_number_arrays = numbers.len();
    let max_x = numbers[0].len();

    for x in 0..max_x {
        let operation = operations[x];
        let mut considered_numbers: Vec<u64> = Vec::new();

        for y in 0..number_of_number_arrays {
            let value = numbers[y][x];
            considered_numbers.push(value);
            // do something with value if needed
        }

        if operation == "+" {
            let sum: u64 = considered_numbers.iter().sum();
            final_sum += sum as u64;
        } else if operation == "*" {
            let product: u64 = considered_numbers.iter().product();
            final_sum += product as u64;
        }
    }

    final_sum
}

fn part2(input: &str) -> u64 {
    let mut numbers_raw: Vec<&str> = vec![];
    let mut operations: Vec<&str> = vec![];
    let mut indices: Vec<(usize, usize)> = vec![];
    let mut final_sum: u64 = 0;

    // iterate through each line
    for raw in input.lines() {
        // access first character to determine if it's an operation or number
        let first_char = raw.chars().next().unwrap();
        let all_chars_from_row: Vec<char> = raw.chars().collect();

        if first_char == '+' || first_char == '*' {
            // process as operation
            operations = raw.split_whitespace().collect();

            // line "*  * *  " -> indices [(0,2), (3,4), (5,7)]
            let mut start_index: Option<usize> = None;
            let all_chars_from_row_len = all_chars_from_row.len();
            for (i, c) in all_chars_from_row.iter().enumerate() {
                if *c != ' ' && start_index.is_none() {
                    start_index = Some(i);
                } else if *c != ' ' && start_index.is_some() {
                    indices.push((start_index.unwrap(), i - 1));
                    start_index = Some(i);
                }

                if i == all_chars_from_row_len - 1 && start_index.is_some() {
                    indices.push((start_index.unwrap(), i));
                }
            }
        } else {
            // process as numbers
            numbers_raw.push(raw);
        }
        // process line
    }

    let number_of_blocks= operations.len();
    let numbers_raw_len = numbers_raw.len();

    for block_index in 0..number_of_blocks {
        // create numbers from top to bottom, right to left
        let mut considered_numbers: Vec<u64> = Vec::new();
        let (start_index, end_index) = indices[block_index];

        for y in (start_index..end_index + 1).rev() {
            let mut number_vec_chars: Vec<char> = vec![];
            for x in 0..numbers_raw_len {
                let c = numbers_raw[x].chars().nth(y).unwrap();
                if c.is_numeric() {
                    number_vec_chars.push(c);
                }
            }
            let number_string: String = number_vec_chars.iter().collect();
            if !number_string.is_empty() {
                let number: u64 = number_string.parse().unwrap();
                considered_numbers.push(number);
            }
        }

        let operation = operations[block_index];
        if operation == "+" {
            let sum: u64 = considered_numbers.iter().sum();
            final_sum += sum as u64;
        } else if operation == "*" {
            let product: u64 = considered_numbers.iter().product();
            final_sum += product as u64;
        }
    }

    final_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = "155\n614\n89 \n18 \n*  "; // Provide a test input
        let result = part2(input);
        assert_eq!(result, 471_843_252); // Provide the expected result
    }
}
