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

fn find_largest_joltage_2(line: &str, window_size: usize) -> u64 {
    let c: Vec<u64> = line.chars()
        .filter_map(|c| c.to_digit(10))
        .map(|x| x as u64)
        .collect();

    let mut pointer_vec: Vec<usize> = Vec::new();
    for i in 0..window_size {
        pointer_vec.push(i);
    }

    let mut max_joltage: u64 = 0;

    while pointer_vec[0] < c.len() - window_size + 1 {
        // calculate joltage
        let mut joltage: u64 = 0;
        
        // itarate over pointer_vec to build the joltage number
        for p in pointer_vec.iter().enumerate() {
            let pointer_index = p.0;
            let pointer_value = *p.1;

            let exponent = (window_size - pointer_index - 1) as u32;

            joltage = joltage + 10_u64.pow(exponent) * c[pointer_value];
            if joltage > max_joltage {
                max_joltage = joltage;
            }
        }

        // set next pointer position
        // increment the last pointer that is possible to increment and reset all the following pointers
        let mut done = true;
        for i in (0..window_size).rev() {
            if pointer_vec[i] < c.len() - (window_size - i) {
                pointer_vec[i] += 1;
                // reset all following pointers
                for j in i+1..window_size {
                    pointer_vec[j] = pointer_vec[j-1] + 1;
                }
                done = false;
                break;
            }
        }

        if done {
            break;
        }
    }

    max_joltage
}

fn part2(input: &str) -> u64 {
    // iterate over each line
    let mut sum: u64 = 0;
    for line in input.lines() {
        sum += find_largest_joltage_2(line, 12);
        println!("Line processed.");
    }

    sum
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
