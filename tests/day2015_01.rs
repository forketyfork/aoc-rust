use adv_code_2024::{day2015_01, read_input};

#[test]
fn day2015_01_real_input() -> anyhow::Result<()> {
    let input = read_input("2015-01")?;
    let part1 = day2015_01::part1(input)?;
    assert_eq!(74, part1);
    Ok(())
}
