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
