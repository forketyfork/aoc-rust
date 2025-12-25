use adv_code_2024::{day2024_03, read_input};

#[test]
fn day2024_03_real_input() -> anyhow::Result<()> {
    let input = read_input("2024-03")?;
    let part1 = day2024_03::part1(input)?;
    assert_eq!(178538786, part1);

    let input = read_input("2024-03")?;
    let part2 = day2024_03::part2(input)?;
    assert_eq!(102467299, part2);
    Ok(())
}
