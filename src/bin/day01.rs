use advent_of_code_2025::read_input;
use std::time::Instant;

fn main() {
    println!("--- Day 1 ---");
    match read_input(1) {
        Ok(input) => {
            let start_part1: Instant = Instant::now();
            let part1_result: usize = part1(&input);
            let part1_duration: std::time::Duration = start_part1.elapsed();
            println!("Part 1: {} (took {:?})", part1_result, part1_duration);

            let start_part2: Instant = Instant::now();
            let part2_result: usize = part2(&input);
            let part2_duration: std::time::Duration = start_part2.elapsed();
            println!("Part 2: {} (took {:?})", part2_result, part2_duration);
        }
        Err(e) => {
            eprintln!("Error reading input: {}", e);
        }
    }
}

fn part1(input: &str) -> usize {
    let start_dial_position: usize = 50;
    let max_dial_position: usize = 99;

    calculate_zero_positions(input, start_dial_position, max_dial_position)
}

fn calculate_zero_positions(
    input: &str,
    start_dial_position: usize,
    max_dial_position: usize,
) -> usize {
    let mut dial_position: isize = start_dial_position.try_into().unwrap();
    let mut count_zero_position: usize = 0;
    let modulo_value: isize = <usize as TryInto<isize>>::try_into(max_dial_position).unwrap() + 1;

    for line in input.lines() {
        let direction: &str = &line[0..1];
        let mut turns: isize = line[1..].parse().unwrap();
        
        if direction == "L" {
            turns *= -1;
        }

        dial_position = (dial_position + turns) % modulo_value;

        if dial_position == 0 {
            count_zero_position += 1;
        }
    }

    count_zero_position
}

fn part2(input: &str) -> usize {
    let start_dial_position: usize = 50;
    let max_dial_position: usize = 99;

    calculate_zero_positions_all(input, start_dial_position, max_dial_position)
}

fn calculate_zero_positions_all(
    input: &str,
    start_dial_position: usize,
    max_dial_position: usize,
) -> usize {
    let mut dial_position: isize = start_dial_position.try_into().unwrap();
    let mut count_zero_position = 0;
    let modulo_value: isize = <usize as TryInto<isize>>::try_into(max_dial_position).unwrap() + 1;

    for line in input.lines() {
        let direction: &str = &line[0..1];
        let mut turns: isize = line[1..].parse().unwrap();

        if direction == "L" {
            turns *= -1;
        }

        let value = calc_how_many_time_passed_zero(dial_position, modulo_value, direction, turns);

        count_zero_position += value;

        dial_position = (dial_position + turns).rem_euclid(modulo_value);
    }

    count_zero_position as usize
}

fn calc_how_many_time_passed_zero(
    dial_position: isize,
    modulo_value: isize,
    direction: &str,
    turns: isize,
) -> isize {
    let value = if direction == "L" {
        let mut val = (dial_position + turns).abs() / modulo_value;
        if turns.abs() >= dial_position && dial_position != 0 {
            val += 1;
        }
        val
    } else {
        // direction == "R"
        ((dial_position + turns) / modulo_value).abs()
    };

    value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_how_many_time_passed_zero_right_8() {
        let dial_position: isize = 0;
        let modulo_value: isize = 100;
        let direction: &str = "R";
        let turns: isize = 200;
        let result = calc_how_many_time_passed_zero(dial_position, modulo_value, direction, turns);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_calc_how_many_time_passed_zero_right_7() {
        let dial_position: isize = 0;
        let modulo_value: isize = 100;
        let direction: &str = "R";
        let turns: isize = 101;
        let result = calc_how_many_time_passed_zero(dial_position, modulo_value, direction, turns);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_calc_how_many_time_passed_zero_right_6() {
        let dial_position: isize = 0;
        let modulo_value: isize = 100;
        let direction: &str = "R";
        let turns: isize = 100;
        let result = calc_how_many_time_passed_zero(dial_position, modulo_value, direction, turns);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_calc_how_many_time_passed_zero_right_5() {
        let dial_position: isize = 0;
        let modulo_value: isize = 100;
        let direction: &str = "R";
        let turns: isize = 100;
        let result = calc_how_many_time_passed_zero(dial_position, modulo_value, direction, turns);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_calc_how_many_time_passed_zero_right_4() {
        let dial_position: isize = 0;
        let modulo_value: isize = 100;
        let direction: &str = "R";
        let turns: isize = 1;
        let result = calc_how_many_time_passed_zero(dial_position, modulo_value, direction, turns);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_calc_how_many_time_passed_zero_right_3() {
        let dial_position: isize = 1;
        let modulo_value: isize = 100;
        let direction: &str = "R";
        let turns: isize = 99;
        let result = calc_how_many_time_passed_zero(dial_position, modulo_value, direction, turns);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_calc_how_many_time_passed_zero_right_2() {
        let dial_position: isize = 1;
        let modulo_value: isize = 100;
        let direction: &str = "R";
        let turns: isize = 98;
        let result = calc_how_many_time_passed_zero(dial_position, modulo_value, direction, turns);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_calc_how_many_time_passed_zero_right_1() {
        let dial_position: isize = 10;
        let modulo_value: isize = 100;
        let direction: &str = "R";
        let turns: isize = 1;
        let result = calc_how_many_time_passed_zero(dial_position, modulo_value, direction, turns);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_calc_how_many_time_passed_zero_left_4() {
        let dial_position: isize = 0;
        let modulo_value: isize = 100;
        let direction: &str = "L";
        let turns: isize = -100;
        let result = calc_how_many_time_passed_zero(dial_position, modulo_value, direction, turns);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_calc_how_many_time_passed_zero_left_3() {
        let dial_position: isize = 0;
        let modulo_value: isize = 100;
        let direction: &str = "L";
        let turns: isize = -1;
        let result = calc_how_many_time_passed_zero(dial_position, modulo_value, direction, turns);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_calc_how_many_time_passed_zero_left_2() {
        let dial_position: isize = 1;
        let modulo_value: isize = 100;
        let direction: &str = "L";
        let turns: isize = -1;
        let result = calc_how_many_time_passed_zero(dial_position, modulo_value, direction, turns);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_calc_how_many_time_passed_zero_left_1() {
        let dial_position: isize = 10;
        let modulo_value: isize = 100;
        let direction: &str = "L";
        let turns: isize = -25;
        let result = calc_how_many_time_passed_zero(dial_position, modulo_value, direction, turns);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_calculate_zero_positions_all_14() {
        let start_dial_position = 0;
        let max_dial_position = 99;
        let input = "L1\nR101\n";
        let result = calculate_zero_positions_all(input, start_dial_position, max_dial_position);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_calculate_zero_positions_all_13() {
        let start_dial_position = 0;
        let max_dial_position = 99;
        let input = "L1\nR1\n";
        let result = calculate_zero_positions_all(input, start_dial_position, max_dial_position);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_calculate_zero_positions_all_12() {
        let start_dial_position = 0;
        let max_dial_position = 99;
        let input = "R1\nL1\nR100\n";
        let result = calculate_zero_positions_all(input, start_dial_position, max_dial_position);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_calculate_zero_positions_all_11() {
        let start_dial_position = 0;
        let max_dial_position = 99;
        let input = "R1\nL1\n";
        let result = calculate_zero_positions_all(input, start_dial_position, max_dial_position);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_calculate_zero_positions_all_10() {
        let start_dial_position = 0;
        let max_dial_position = 99;
        let input = "R1\nL2\n";
        let result = calculate_zero_positions_all(input, start_dial_position, max_dial_position);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_calculate_zero_positions_all_9() {
        let start_dial_position = 50;
        let max_dial_position = 99;
        let input = "R150\n";
        let result = calculate_zero_positions_all(input, start_dial_position, max_dial_position);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_calculate_zero_positions_all_8() {
        let start_dial_position = 50;
        let max_dial_position = 99;
        let input = "R50\n";
        let result = calculate_zero_positions_all(input, start_dial_position, max_dial_position);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_calculate_zero_positions_all_7() {
        let start_dial_position = 50;
        let max_dial_position = 99;
        let input = "R49\n";
        let result = calculate_zero_positions_all(input, start_dial_position, max_dial_position);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_calculate_zero_positions_all_6() {
        let start_dial_position = 0;
        let max_dial_position = 99;
        let input = "R101\n";
        let result = calculate_zero_positions_all(input, start_dial_position, max_dial_position);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_calculate_zero_positions_all_5() {
        let start_dial_position = 0;
        let max_dial_position = 99;
        let input = "R100\n";
        let result = calculate_zero_positions_all(input, start_dial_position, max_dial_position);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_calculate_zero_positions_all_4() {
        let start_dial_position = 0;
        let max_dial_position = 99;
        let input = "R1\n";
        let result = calculate_zero_positions_all(input, start_dial_position, max_dial_position);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_calculate_zero_positions_all_3() {
        let start_dial_position = 2;
        let max_dial_position = 99;
        let input = "L102\n";
        let result = calculate_zero_positions_all(input, start_dial_position, max_dial_position);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_calculate_zero_positions_all_2() {
        let start_dial_position = 2;
        let max_dial_position = 99;
        let input = "L1\n";
        let result = calculate_zero_positions_all(input, start_dial_position, max_dial_position);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_calculate_zero_positions_all_1() {
        let start_dial_position = 1;
        let max_dial_position = 99;
        let input = "L2\n";
        let result = calculate_zero_positions_all(input, start_dial_position, max_dial_position);
        assert_eq!(result, 1);
    }

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
        let input = "R99\nR1\n";
        let result = calculate_zero_positions(input, start_dial_position, max_dial_position);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_calculate_zero_positions_underflow() {
        let start_dial_position = 0;
        let max_dial_position = 99;
        let input = "L1\nL99\n";
        let result = calculate_zero_positions(input, start_dial_position, max_dial_position);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_calculate_zero_positions_all_underflow() {
        let start_dial_position = 0;
        let max_dial_position = 99;
        let input = "L250\n";
        let result = calculate_zero_positions_all(input, start_dial_position, max_dial_position);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_calculate_zero_positions_all_overflow() {
        let start_dial_position = 0;
        let max_dial_position = 99;
        let input = "R250\n";
        let result = calculate_zero_positions_all(input, start_dial_position, max_dial_position);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_calculate_zero_positions_all_under_then_overflow() {
        let start_dial_position = 0;
        let max_dial_position = 99;
        let input = "L300\nR300\n";
        let result = calculate_zero_positions_all(input, start_dial_position, max_dial_position);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_part2() {
        let input = ""; // Provide a test input
        let result = part2(input);
        assert_eq!(result, 0); // Provide the expected result
    }
}
