use advent_of_code_2025::read_input;
use std::time::Instant;

fn main() {
    println!("--- Day 5 ---");
    match read_input(5) {
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
    // iterate over lines until empty line, after that switch processing mode
    #[derive(Copy, Clone, Debug)]
    enum Mode {
        Ranges,
        Ingredients,
    }

    let mut mode = Mode::Ranges;
    let mut sum_of_fresh_ingredients: usize = 0;

    // vector of id ranges
    let mut ranges: Vec<(u64, u64)> = Vec::new();
    let mut simplified_ranges: Vec<(u64, u64)> = Vec::new();

    for raw in input.lines() {
        let line = raw.trim();
        if line.is_empty() {
            mode = Mode::Ingredients;
            // simplify ranges
            simplified_ranges = simplify_ranges(&ranges);
            continue;
        }

        match mode {
            Mode::Ranges => {
                // parse line like "1-3 or 5-7"
                let parts: Vec<u64> = line.split("-")
                .map(|s| s.parse().unwrap())
                .collect();
                ranges.push((parts[0], parts[1]));
            }
            Mode::Ingredients => {
                let ingredient_id: u64 = line.parse().unwrap();
                if is_ingredient_fresh(ingredient_id, &simplified_ranges) {
                    sum_of_fresh_ingredients += 1;
                }
            }
        }
    }

    sum_of_fresh_ingredients
}


fn is_ingredient_fresh(ingredient_id: u64, ranges: &Vec<(u64, u64)>) -> bool {
    for (start, end) in ranges {
        if ingredient_id >= *start && ingredient_id <= *end {
            return true;
        }
    }
    false
}

fn part2(input: &str) -> usize {
    // iterate over lines until empty line, after that switch processing mode
    let mut sum_of_fresh_ingredients: usize = 0;

    // vector of id ranges
    let mut ranges: Vec<(u64, u64)> = Vec::new();

    for raw in input.lines() {
        let line = raw.trim();
        if line.is_empty() {
            break;
        }

        // parse line like "1-3 or 5-7"
        let parts: Vec<u64> = line.split("-")
        .map(|s| s.parse().unwrap())
        .collect();
        ranges.push((parts[0], parts[1]));
         
    }

    // simplify ranges
    let simplified_ranges = simplify_ranges(&ranges);

    // iterate over ranges to calculate total fresh ingredients
    for (start, end) in simplified_ranges {
        sum_of_fresh_ingredients += (end - start + 1) as usize;
    }

    sum_of_fresh_ingredients
}


fn simplify_ranges(ranges: &Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    let mut ranges = ranges.clone();
    ranges.sort_by_key(|(start, _end)| *start);

    let mut simplified: Vec<(u64, u64)> = Vec::new();

    for (start, end) in ranges {
        if let Some((_s, e)) = simplified.last_mut() {
            if start <= e.saturating_add(1) {
                *e = (*e).max(end);
                continue;
            }
        }

        simplified.push((start, end));
    }

    simplified
}

#[cfg(test)]
mod tests {
    use super::*;

    // test simplify_ranges
    #[test]
    fn test_simplify_ranges_2() {
        let ranges = vec![(2, 4), (1, 3)];
        let simplified = simplify_ranges(&ranges);
        assert_eq!(simplified, vec![(1, 4)]);
    }

    #[test]
    fn test_simplify_ranges_1() {
        let ranges = vec![(1, 3), (2, 4)];
        let simplified = simplify_ranges(&ranges);
        assert_eq!(simplified, vec![(1, 4)]);
    }

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
