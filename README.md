# Advent of Code 2025

This repository contains solutions for the Advent of Code 2025 challenges, written in Rust.

## Project Structure

The project is organized into a single package with multiple binaries, one for each day's solution.

- **`src/bin/`**: Contains the source code for each day's solution (e.g., `day01.rs`, `day02.rs`).
- **`src/lib.rs`**: A shared library for common functions (e.g., input reading) used across different days.
- **`input/`**: Stores the puzzle input files for each day (e.g., `day01.txt`).
- **`tasks/`**: Stores the markdown problem descriptions for each day (e.g., `day01.md`).

## How to Run a Specific Day's Solution

To run the code for a specific day, use the `--bin` flag with `cargo run`. For example, to run the solution for Day 1, execute the following command in your terminal:

```sh
cargo run --bin day01
```

Replace `day01` with the appropriate day you want to run (e.g., `day02`, `day03`, etc.).

## How to Run All Solutions

To run all available solutions in succession, you can use the `run_all.sh` script:

```sh
./run_all.sh
```

This will execute each day's binary and print the results.

## How to Test Solutions

You can run all tests for the entire project using:

```sh
cargo test
```

To test a specific day's solution, use the `--bin` flag. For example, to test the solution for Day 1:

```sh
cargo test --bin day01
```

## Current results + timing results (SPOILERS!)

### --- Day 1 ---

Part 1: 982 (took 36.875µs)

Part 2: 6106 (took 75µs)

### --- Day 2 ---

Part 1: 5398419778 (took 8.175833ms)

Part 2: 15704845910 (took 206.530584ms)

### --- Day 3 ---

Part 1: 17179 (took 138.959µs)

Part 2: 170025781683941 (took 127µs)

### --- Day 4 ---

Part 1: 1474 (took 231.875µs)

Part 2: 8910 (took 3.871375ms)

### --- Day 5 ---

Part 1: 640 (took 74.167µs)

Part 2: 365804144481581 (took 28.667µs)

### --- Day 6 ---

Part 1: 5552221122013 (took 257.875µs)

Part 2: 11371597126232 (took 3.598417ms)
