use adv_code_2024::{day2024_04, read_input};

#[test]
#[ignore]
fn day2024_04_real_input() -> anyhow::Result<()> {
    let input = read_input("2024-04")?;
    let part1 = day2024_04::part1(input)?;
    assert_eq!(2370, part1);

    let input = read_input("2024-04")?;
    let part2 = day2024_04::part2(input)?;
    assert_eq!(1908, part2);
    Ok(())
}
