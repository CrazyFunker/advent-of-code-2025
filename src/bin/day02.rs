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
    input.lines().count()
}

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
