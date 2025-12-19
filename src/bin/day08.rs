use advent_of_code_2025::read_input;
use std::time::Instant;

fn main() {
    println!("--- Day 8 ---");
    match read_input(8) {
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

#[derive(Debug, Clone, Copy)]
struct Junction {
    id: usize,
    x: u64,
    y: u64,
    z: u64,
    closest_junction_id: Option<usize>,
    closest_junction_distance_sq: Option<u64>,
    circuit_id: u32,
}

fn calc_junctions_distance_sq(a: &Junction, b: &Junction) -> u64 {
    let dx = a.x as i64 - b.x as i64;
    let dy = a.y as i64 - b.y as i64;
    let dz = a.z as i64 - b.z as i64;

    (dx * dx + dy * dy + dz * dz) as u64
}

fn find_and_set_closest_junction_information(junctions: &mut [Junction]) {
    for i in 0..junctions.len() {
        let closest_junction_id = find_closest_junction_id(i, junctions);
        match closest_junction_id {
            Some(j) => {
                let distance_sq = calc_junctions_distance_sq(&junctions[i], &junctions[j]);
                junctions[i].closest_junction_id = Some(j);
                junctions[i].closest_junction_distance_sq = Some(distance_sq);
            },
            None => {
                junctions[i].closest_junction_id = None;
                junctions[i].closest_junction_distance_sq = None;
            },
        }
    }
}

fn find_closest_junction_id(junction_id: usize, junctions: &[Junction]) -> Option<usize> {
    junctions
        .iter()
        // exclude the exact same junction (same coordinates)
        .filter(|j| !(j.x == junctions[junction_id].x && j.y == junctions[junction_id].y && j.z == junctions[junction_id].z))
        .min_by_key(|j| calc_junctions_distance_sq(&junctions[junction_id], j))
        .map(|j| j.id)
}

fn find_closest_not_directly_connected_junction_id(junction_id: usize, junctions: &[Junction]) -> Option<usize> {
    junctions
        .iter()
        // exclude the exact same junction (same coordinates)
        .filter(|j| !(j.x == junctions[junction_id].x && j.y == junctions[junction_id].y && j.z == junctions[junction_id].z))
        // exclude junctions that are in the same circuit
        .filter(|j| j.circuit_id != junctions[junction_id].circuit_id || junctions[junction_id].circuit_id == 0)
        .min_by_key(|j| calc_junctions_distance_sq(&junctions[junction_id], j))
        .map(|j| j.id)
}

fn part1(input: &str) -> u64 {
    let mut junctions_positions: Vec<Junction> = vec![];
    let mut junction_id = 0;
    
    for raw in input.lines() {
        // process line
        let id = junction_id;
        let num_str: Vec<&str> = raw.split(',').collect();
        let x = num_str[0].parse::<u64>().unwrap();
        let y = num_str[1].parse::<u64>().unwrap();
        let z = num_str[2].parse::<u64>().unwrap();
        let closest_junction_id = None;
        let closest_junction_distance_sq = None;
        let circuit_id = 0;

        junctions_positions.push(Junction {
            id,
            x,
            y,
            z,
            closest_junction_id,
            closest_junction_distance_sq,
            circuit_id,
        });

        junction_id += 1;
    }

    // Calculate closest junction for each junction
    find_and_set_closest_junction_information(&mut junctions_positions);

    let mut sorted_junctions = junctions_positions.clone();

    // sort junctions_poisitions ascending by closest_junction_distance_sq
    sorted_junctions.sort_by(|a, b| a.closest_junction_distance_sq.cmp(&b.closest_junction_distance_sq));

    let mut new_circuit_id = 1;
    for i in 0..sorted_junctions.len() {
        let ij_id = sorted_junctions[i].id;

        if junctions_positions[ij_id].circuit_id != 0 {
            println!("Skipping Junction: ({}, {}, {}), Circuit ID: {}",
                junctions_positions[ij_id].x, junctions_positions[ij_id].y, junctions_positions[ij_id].z,
                junctions_positions[ij_id].circuit_id
            );
            // already assigned to a circuit
            continue;
        }

        let closest_junction_id: Option<usize> = find_closest_not_directly_connected_junction_id(ij_id, &junctions_positions);
        // let closest_junction_id: Option<usize> = sorted_junctions[i].closest_junction_id;
        match closest_junction_id {
            Some(cj_id) =>  {
                println!("Calculating for Junction: ({}, {}, {}), Closest: ({}, {}, {})\t Circuit IDs: {} and {}", 
                    junctions_positions[ij_id].x, junctions_positions[ij_id].y, junctions_positions[ij_id].z,
                    junctions_positions[cj_id].x, junctions_positions[cj_id].y, junctions_positions[cj_id].z,
                    junctions_positions[ij_id].circuit_id, junctions_positions[cj_id].circuit_id
                );
                if junctions_positions[ij_id].circuit_id == 0 && junctions_positions[cj_id].circuit_id == 0 {
                    println!("1: Neither junction has a circuit_id, assigning new circuit_id {}", new_circuit_id);
                    junctions_positions[ij_id].circuit_id = new_circuit_id;
                    junctions_positions[cj_id].circuit_id = new_circuit_id;
                    new_circuit_id += 1;
                } else if junctions_positions[ij_id].circuit_id == junctions_positions[cj_id].circuit_id {
                    println!("2: Both junctions already have the same circuit_id {}", junctions_positions[ij_id].circuit_id);
                } else if junctions_positions[ij_id].circuit_id != 0 && junctions_positions[cj_id].circuit_id != 0 {
                    println!("3: Both junctions have different circuit_ids: {} and {}, merging circuits", 
                        junctions_positions[ij_id].circuit_id, junctions_positions[cj_id].circuit_id);
                    // need to merge circuits
                    let old_id = junctions_positions[cj_id].circuit_id;
                    let new_id = junctions_positions[ij_id].circuit_id;
                    // println!("Merging circuits: {} -> {}", old_id, new_id);
                    for other_junction in &mut junctions_positions {
                        if other_junction.circuit_id == old_id {
                            other_junction.circuit_id = new_id;
                        }
                    }
                } else {
                    let id_to_assign = junctions_positions[ij_id].circuit_id + junctions_positions[cj_id].circuit_id;
                    println!("4: Only one of the junctions has a circuit_id, will assign circuit_id {} to both junctions", id_to_assign);
                    junctions_positions[ij_id].circuit_id = id_to_assign;
                    junctions_positions[cj_id].circuit_id = id_to_assign;
                    // println!("Assigning circuit_id {} to both junctions", id_to_assign);
                }
            },
            None => println!("Junction: {:?}, Closest: None", junctions_positions[ij_id]),
        }

        // calculate number of unique circuit_ids and how many junctions belong to each circuit
        let mut circuit_counts: std::collections::HashMap<u32, i32> = std::collections::HashMap::new();
        for junction in &junctions_positions {
            *circuit_counts.entry(junction.circuit_id).or_insert(0) += 1;
        }
        println!("Circuit counts so far: {:?}\n", circuit_counts);

    }

    // calculate number of unique circuit_ids and how many junctions belong to each circuit
    let mut circuit_counts: std::collections::HashMap<u32, i32> = std::collections::HashMap::new();
    for junction in &junctions_positions {
        *circuit_counts.entry(junction.circuit_id).or_insert(0) += 1;
    }

    println!("Circuit counts: {:?}", circuit_counts);

    // order circuit_counts descending by entry key
    let mut circuit_counts_vec: Vec<(&u32, &i32)> = circuit_counts.iter().collect();
    circuit_counts_vec.sort_by(|a, b| b.1.cmp(a.1));
    println!("Ordered circuit counts: {:?}", circuit_counts_vec);

    let first_count = circuit_counts_vec[0].1;
    let second_count = circuit_counts_vec[1].1; 
    let third_count = circuit_counts_vec[2].1;

    println!("Top 3 circuit counts: {}, {}, {}", first_count, second_count, third_count);

    (first_count * second_count * third_count) as u64
}

fn part2(input: &str) -> usize {
    input.chars().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
        let result = part1(input);
        assert_eq!(result, 40); // Replace 0 with the expected result
    }

}
