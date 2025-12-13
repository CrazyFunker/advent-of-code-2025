use advent_of_code_2025::read_input;
use std::time::Instant;

fn main() {
    println!("--- Day 4 ---");
    match read_input(4) {
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
    let mut grid: Vec<Vec<bool>> = Vec::new();
    for line in input.lines() {
        let row: Vec<bool> = line.chars().map(|c| c == '@').collect();
        grid.push(row);
    }

    how_many_rolls_can_be_accessed(&grid)
}

fn how_many_rolls_can_be_accessed(grid: &Vec<Vec<bool>>) -> usize {
    let mut count = 0;
    let max_y = grid.len();
    let max_x = grid[0].len();

    for y in 0..max_y {
        for x in 0..max_x {
            if grid[y][x] && count_neighbors(grid, x, y) < 4 {
                count += 1;
            }
        }
    }

    count
}

fn count_neighbors(grid: &Vec<Vec<bool>>, x: usize, y: usize) -> usize {
    let directions = [(-1, -1), (-1, 0), (-1, 1),
                                           (0, -1),            (0, 1),
                                           (1, -1),  (1, 0),  (1, 1)];
    let mut count = 0;
    let max_y = grid.len() as isize;
    let max_x = grid[0].len() as isize;

    for (dx, dy) in directions.iter() {
        let nx = x as isize + dx;
        let ny = y as isize + dy;
        if nx >= 0 && nx < max_x && ny >= 0 && ny < max_y {
            if grid[ny as usize][nx as usize] {
                count += 1;
            }
        }
    }

    count
}

fn part2(input: &str) -> usize {
    let mut grid: Vec<Vec<bool>> = Vec::new();
    for line in input.lines() {
        let row: Vec<bool> = line.chars().map(|c| c == '@').collect();
        grid.push(row);
    }

    let mut total_removed: usize = 0;
    let mut removed: usize = 1;

    while removed > 0 {
        removed = remove_rolls(&mut grid);
        total_removed += removed;
    }

    total_removed
}

fn remove_rolls(grid: &mut Vec<Vec<bool>>) -> usize {
    let mut count = 0;
    let mut for_removal: Vec<(usize, usize)> = Vec::new();
    let max_y = grid.len();
    let max_x = grid[0].len();

    for y in 0..max_y {
        for x in 0..max_x {
            if grid[y][x] && count_neighbors(grid, x, y) < 4 {
                count += 1;
                for_removal.push((x, y));
            }
        }
    }

    for (x, y) in for_removal {
        grid[y][x] = false;
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count_neighbors2() {
        let grid = vec![
            vec![true, true, false],
            vec![true, true, false],
            vec![false, false, false],
        ];
        assert_eq!(count_neighbors(&grid, 0, 0), 3);
    }

    #[test]
    fn test_count_neighbors1() {
        let grid = vec![
            vec![false, true, false],
            vec![true, true, false],
            vec![false, false, false],
        ];
        assert_eq!(count_neighbors(&grid, 1, 1), 2);
    }
       
}
