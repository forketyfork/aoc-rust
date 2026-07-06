use adv_code_2024::{day2024_05, read_input};

#[cfg_attr(ci_environment, ignore)]
#[test]
fn day2024_05_real_input() -> anyhow::Result<()> {
    let input = read_input("2024-05")?;
    let part1 = day2024_05::part1(input)?;
    assert_eq!(4662, part1);

    Ok(())
}
