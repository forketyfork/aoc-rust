use adv_code_2024::{day2024_02, read_input};

#[test]
fn day2024_02_real_input() -> anyhow::Result<()> {
    let input = read_input("2024-02")?;
    let part1 = day2024_02::part1(input)?;
    assert_eq!(510, part1);

    let input = read_input("2024-02")?;
    let part2 = day2024_02::part2(input)?;
    assert_eq!(553, part2);
    Ok(())
}
