use adv_code_2024::{day2024_01, read_input};

#[cfg_attr(ci_environment, ignore)]
#[test]
fn day2024_01_real_input() -> anyhow::Result<()> {
    let input = read_input("2024-01")?;
    let part1 = day2024_01::part1(input)?;
    assert_eq!(2264607, part1);

    let input = read_input("2024-01")?;
    let part2 = day2024_01::part2(input)?;
    assert_eq!(19457120, part2);
    Ok(())
}
