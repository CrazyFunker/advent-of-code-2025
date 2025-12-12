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
    input.lines().map(find_largest_joltage).sum()
}

fn find_largest_joltage(line: &str) -> usize {
    max_joltage_subsequence(line, 2) as usize
}

fn find_largest_joltage_2(line: &str, digits_to_pick: usize) -> u64 {
    max_joltage_subsequence(line, digits_to_pick)
}

fn max_joltage_subsequence(line: &str, k: usize) -> u64 {
    // Keep prior behavior: ignore non-digits.
    let digits: Vec<u8> = line
        .bytes()
        .filter(|b| b.is_ascii_digit())
        .map(|b| b - b'0')
        .collect();

    if k == 0 || k > digits.len() {
        return 0;
    }

    // Greedy monotonic stack: remove (n-k) digits to maximize the remaining k-digit number.
    let mut remove = digits.len() - k;
    let mut stack: Vec<u8> = Vec::with_capacity(digits.len());
    for d in digits {
        while remove > 0 {
            match stack.last() {
                Some(&last) if last < d => {
                    stack.pop();
                    remove -= 1;
                }
                _ => break,
            }
        }
        stack.push(d);
    }

    if remove > 0 {
        stack.truncate(stack.len().saturating_sub(remove));
    }
    stack.truncate(k);

    let mut value: u64 = 0;
    for d in stack {
        value = value
            .checked_mul(10)
            .and_then(|v| v.checked_add(d as u64))
            .expect("joltage number overflowed u64");
    }
    value
}

fn part2(input: &str) -> u64 {
    input.lines().map(|line| find_largest_joltage_2(line, 12)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_largest_joltage_2_1() {
        let input = "123";
        assert_eq!(find_largest_joltage_2(input, 3), 123);
    }

    #[test]
    fn test_find_largest_joltage_987654321111111() {
        let input = "987654321111111";
        assert_eq!(find_largest_joltage_2(input, 12), 987654321111);
    }

    #[test]
    fn test_find_largest_joltage_811111111111119() {
        let input = "811111111111119";
        assert_eq!(find_largest_joltage_2(input, 12), 811111111119);
    }

    #[test]
    fn test_find_largest_joltage_234234234234278() {
        let input = "234234234234278";
        assert_eq!(find_largest_joltage_2(input, 12), 434234234278);
    }

    #[test]
    fn test_find_largest_joltage_818181911112111() {
        let input = "818181911112111";
        assert_eq!(find_largest_joltage_2(input, 12), 888911112111);
    }

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
