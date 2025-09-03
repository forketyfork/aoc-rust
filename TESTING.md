# Suggested Improvements to the Testing Framework

This repository currently places assertions directly inside each day's binary. While this
works for quick checks during development, a more structured testing approach can improve
maintainability and automation. Below are some proposals:

## 1. Use standard Rust tests
- Move day-specific assertions into `#[cfg(test)]` modules within each file or
  into `tests/` integration tests. This allows running all checks with `cargo test`.
- Keep `main` focused on solving the puzzle and printing results.

## 2. Add reusable helpers
- Provide helper functions/macros in `src/lib.rs` to read inputs and run
  `part1`/`part2` functions.
- Parameterize these helpers so tests remain concise and consistent across days.

## 3. Support property-based testing
- Consider integrating crates such as `proptest` or `quickcheck` to explore edge
  cases automatically.
- Property-based tests work well for verifying smaller helper functions used in
the solutions.

## 4. Integration tests with real inputs
- Place final puzzle answers in integration tests (e.g. `tests/day01.rs`).
- These tests can load data from the `input` directory and assert the known
  answers, ensuring refactors don't break previous days.

## 5. Continuous integration
- Configure CI (GitHub Actions or similar) to execute `cargo test` on every pull
  request, preventing regressions.

These changes would make the project easier to maintain, encourage reuse, and
provide faster feedback when solving new puzzles.
