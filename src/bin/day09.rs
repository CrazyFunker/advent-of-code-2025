use advent_of_code_2025::read_input;

fn main() {
    println!("--- Day 9 ---");
    match read_input(9) {
        Ok(input) => {
            // Part 1
            println!("Part 1: {}", input.lines().count());
            // Part 2
            println!("Part 2: {}", input.chars().count());
        }
        Err(e) => {
            eprintln!("Error reading input: {}", e);
        }
    }
}
