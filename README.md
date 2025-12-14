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

## Current results + timing results (🚨 SPOILERS 🚨)

### Timing Results

| Day | Part 1 time | Part 2 time |
| --- | --- | --- |
| 1 | 36.875µs | 75µs |
| 2 | 8.175833ms | 206.530584ms |
| 3 | 138.959µs | 127µs |
| 4 | 231.875µs | 3.871375ms |
| 5 | 74.167µs | 28.667µs |
| 6 | 257.875µs | 3.598417ms |
| 7 | 19.5µs | 24.542µs |

### Final Results (🚨 SPOILERS 🚨)

| Day | Part 1 | Part 2 |
| --- | --- | --- |
| 1 | 982 | 6106 |
| 2 | 5398419778 | 15704845910 |
| 3 | 17179 | 170025781683941 |
| 4 | 1474 | 8910 |
| 5 | 640 | 365804144481581 |
| 6 | 5552221122013 | 11371597126232 |
| 7 | 1717 | 231507396180012 |
