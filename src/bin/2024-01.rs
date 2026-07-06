use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;

const DAY: &str = "2024-01";

fn main() -> Result<()> {
    start_day(DAY);

    println!("=== Part 1 ===");
    let input = read_input(DAY)?;
    let result = time_snippet!(day2024_01::part1(input)?);
    println!("Result = {}", result);

    println!("\n=== Part 2 ===");
    let input = read_input(DAY)?;
    let result = time_snippet!(day2024_01::part2(input)?);
    println!("Result = {}", result);

    Ok(())
}
