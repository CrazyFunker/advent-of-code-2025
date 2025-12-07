use advent_of_code_2025::read_input;
use std::time::Instant;

fn main() {
    println!("--- Day 1 ---");
    match read_input(1) {
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
    let start_dial_position = 50;
    let max_dial_position = 99;
    let min_dial_position = 0;

    calculate_zero_positions(input, start_dial_position, min_dial_position, max_dial_position)
}

fn calculate_zero_positions(input: &str, mut start_dial_position: usize, min_dial_position: usize, max_dial_position: usize) -> usize {
    let mut dial_position: isize = start_dial_position.try_into().unwrap();
    let mut count_zero_position = 0;
    let modulo_value: isize = <usize as TryInto<isize>>::try_into(max_dial_position).unwrap() + 1;
    // loop over lines
    // each line contains direction of dial turn
    // L68 = turn left 68 times
    // R23 = turn right 23 times
    for line in input.lines() {
        // println!("{}", line);
        let direction = &line[0..1];
        let mut turns: isize = line[1..].parse().unwrap();
        // println!("{}{} ", direction, turns);
        match direction {
            "L" => {
                turns *= -1;
            }
            "R" => {
                // No operation needed for right turns
            }
            _ => {
                println!("Invalid direction");
            }
        }

        dial_position = (dial_position + turns) % modulo_value;

        if dial_position == 0 {
            // println!("Dial at zero position!");
            count_zero_position += 1;
        }
    }

    // final dial position
    println!("Final dial position: {}", dial_position);

    count_zero_position
}

fn part2(input: &str) -> usize {
    input.chars().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "R1\nL1";
        let result = part1(input);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_calculate_zero_positions_overflow() {
        let start_dial_position = 0;
        let max_dial_position = 99;
        let min_dial_position = 0;
        let input = "R99\nR1\n";
        let result = calculate_zero_positions(input, start_dial_position, min_dial_position, max_dial_position);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_calculate_zero_positions_underflow() {
        let start_dial_position = 0;
        let max_dial_position = 99;
        let min_dial_position = 0;
        let input = "L1\nL99\n";
        let result = calculate_zero_positions(input, start_dial_position, min_dial_position, max_dial_position);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_part2() {
        let input = ""; // Provide a test input
        let result = part2(input);
        assert_eq!(result, 0); // Provide the expected result
    }
}
