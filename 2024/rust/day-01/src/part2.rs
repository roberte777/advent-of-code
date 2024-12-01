use crate::part1::create_lists;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (left_list, right_list) = create_lists(input);
    Ok(left_list
        .into_iter()
        .map(|lv| right_list.iter().filter(|rv| **rv == lv).count() as u32 * lv)
        .sum::<u32>()
        .to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!("31", process(input)?);
        Ok(())
    }
}
