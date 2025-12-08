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
            if is_number_an_invalid_id_part1(number) {
                invalid_ids.push(number);
            }
        }

        sum_of_invalid_ids = sum_of_invalid_ids + invalid_ids.iter().sum::<u64>();
    }

    sum_of_invalid_ids
}

fn is_number_an_invalid_id_part1(number: u64) -> bool {
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
        let decimal_multiplier: u64 = 10_u64.pow(digit_count / 2);
        let first_half: u64 = number / decimal_multiplier;
        let second_half: u64 = number - first_half * decimal_multiplier;
        
        if first_half == second_half {
            return true;
        }
    }
    false
}

fn part2(input: &str) -> u64 {
    // precalculate divisors for digit counts 1 to 20
    let _divisors_map: Vec<Vec<usize>> = (1..=20)
        .map(|digit_count| {
            let mut divisors: Vec<usize> = vec![];
            for i in 1..=digit_count {
                if digit_count % i == 0 {
                    divisors.push(i);
                }
            }
            divisors
        })
        .collect();

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
            if is_number_an_invalid_id_part2(number, &_divisors_map) {
                invalid_ids.push(number);
            }
        }

        sum_of_invalid_ids = sum_of_invalid_ids + invalid_ids.iter().sum::<u64>();
    }

    sum_of_invalid_ids
}

fn is_number_an_invalid_id_part2(number: u64, _divisors_map: &Vec<Vec<usize>>) -> bool {
    // calculate the number of digits in the number, without converting to string
    let n = number;
    // convert number to string
    let number_str = n.to_string();
    let digit_count = number_str.len();

    // fetch divisors from precomputed map
    let divisors = &_divisors_map[digit_count - 1];

    // iterate over divisors
    for divisor in divisors.iter().take(divisors.len() - 1) {
        // chunk the number string into parts of size divisor
        let chunks: Vec<&str> = number_str.as_bytes()
            .chunks(*divisor)
            .map(|chunk| std::str::from_utf8(chunk).unwrap())
            .collect();
        // check if all chunks are the same
        let first_chunk = chunks[0];
        if chunks.iter().all(|&chunk| chunk == first_chunk) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use once_cell::sync::Lazy;

    static DIVISORS_MAP: Lazy<Vec<Vec<usize>>> = Lazy::new(|| {
        (1..=20)
            .map(|digit_count| {
                let mut divisors: Vec<usize> = vec![];
                for i in 1..=digit_count {
                    if digit_count % i == 0 {
                        divisors.push(i);
                    }
                }
                divisors
            })
            .collect()
    });

    #[test]
    fn test_invalid_id_part2_11() {
        assert_eq!(is_number_an_invalid_id_part2(11, &DIVISORS_MAP), true);
    }

    #[test]
    fn test_invalid_id_part2_22() {
        assert_eq!(is_number_an_invalid_id_part2(22, &DIVISORS_MAP), true);
    }

    #[test]
    fn test_invalid_id_part2_99() {
        assert_eq!(is_number_an_invalid_id_part2(99, &DIVISORS_MAP), true);
    }

    #[test]
    fn test_invalid_id_part2_111() {
        assert_eq!(is_number_an_invalid_id_part2(111, &DIVISORS_MAP), true);
    }

    #[test]
    fn test_invalid_id_part2_999() {
        assert_eq!(is_number_an_invalid_id_part2(999, &DIVISORS_MAP), true);
    }

    #[test]
    fn test_invalid_id_part2_1010() {
        assert_eq!(is_number_an_invalid_id_part2(1010, &DIVISORS_MAP), true);
    }

    #[test]
    fn test_invalid_id_part2_1188511885() {
        assert_eq!(is_number_an_invalid_id_part2(1188511885, &DIVISORS_MAP), true);
    }

    #[test]
    fn test_invalid_id_part2_222222() {
        assert_eq!(is_number_an_invalid_id_part2(222222, &DIVISORS_MAP), true);
    }

    #[test]
    fn test_invalid_id_part2_446446() {
        assert_eq!(is_number_an_invalid_id_part2(446446, &DIVISORS_MAP), true);
    }

    #[test]
    fn test_invalid_id_part2_38593859() {
        assert_eq!(is_number_an_invalid_id_part2(38593859, &DIVISORS_MAP), true);
    }

    #[test]
    fn test_invalid_id_part2_565656() {
        assert_eq!(is_number_an_invalid_id_part2(565656, &DIVISORS_MAP), true);
    }

    #[test]
    fn test_invalid_id_part2_824824824() {
        assert_eq!(is_number_an_invalid_id_part2(824824824, &DIVISORS_MAP), true);
    }

    #[test]
    fn test_invalid_id_part2_2121212121() {
        assert_eq!(is_number_an_invalid_id_part2(2121212121, &DIVISORS_MAP), true);
    }

    #[test]
    fn test_valid_id_part2_95() {
        assert_eq!(is_number_an_invalid_id_part2(95, &DIVISORS_MAP), false);
    }

    #[test]
    fn test_valid_id_part2_1698522() {
        assert_eq!(is_number_an_invalid_id_part2(1698522, &DIVISORS_MAP), false);
    }

    #[test]
    fn test_valid_id_part2_1698528() {
        assert_eq!(is_number_an_invalid_id_part2(1698528, &DIVISORS_MAP), false);
    }

    #[test]
    fn test_invalid_id_11() {
        assert_eq!(is_number_an_invalid_id_part1(11), true);
    }

    #[test]
    fn test_invalid_id_22() {
        assert_eq!(is_number_an_invalid_id_part1(22), true);
    }

    #[test]
    fn test_invalid_id_99() {
        assert_eq!(is_number_an_invalid_id_part1(99), true);
    }

    #[test]
    fn test_invalid_id_1010() {
        assert_eq!(is_number_an_invalid_id_part1(1010), true);
    }

    #[test]
    fn test_invalid_id_1188511885() {
        assert_eq!(is_number_an_invalid_id_part1(1188511885), true);
    }

    #[test]
    fn test_invalid_id_222222() {
        assert_eq!(is_number_an_invalid_id_part1(222222), true);
    }

    #[test]
    fn test_invalid_id_446446() {
        assert_eq!(is_number_an_invalid_id_part1(446446), true);
    }

    #[test]
    fn test_invalid_id_38593859() {
        assert_eq!(is_number_an_invalid_id_part1(38593859), true);
    }

    #[test]
    fn test_valid_id_95() {
        assert_eq!(is_number_an_invalid_id_part1(95), false);
    }

    #[test]
    fn test_valid_id_1698522() {
        assert_eq!(is_number_an_invalid_id_part1(1698522), false);
    }

    #[test]
    fn test_valid_id_1698528() {
        assert_eq!(is_number_an_invalid_id_part1(1698528), false);
    }
}
