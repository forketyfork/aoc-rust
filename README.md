# Advent of Code Solutions in Rust

[![Build status](https://github.com/forketyfork/aoc-rust/actions/workflows/build.yml/badge.svg)](https://github.com/forketyfork/aoc-rust/actions/workflows/build.yml)
[![MIT License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/language-Rust-orange.svg)](https://www.rust-lang.org/)

This repository contains my solutions to [Advent of Code](https://adventofcode.com/) puzzles written in Rust.

## Structure

- `src/bin/` - Individual solution files for each day (e.g., `01.rs`, `02.rs`, etc.)
- `input/` - Input data files for each day's puzzle
- Each solution includes both Part 1 and Part 2 of the daily challenges

## Running Solutions

To run a specific day's solution:

```bash
cargo run --bin YYYY-DD
```

For example, to run the solution for Day 5 of 2024:

```bash
cargo run --bin 2024-05
```

## Building and Testing

```bash
# Build all solutions
cargo build

# Run all tests
cargo test

# Check code formatting
cargo fmt --check

# Run clippy for linting
cargo clippy
```
