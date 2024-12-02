use crate::part1::parse_input;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let reports = parse_input(input);
    let num_safe = reports
        .into_iter()
        .map(|r| r.is_safe_2())
        .fold(0, |acc, x| {
            dbg!(acc, x);
            if x {
                return acc + 1;
            }
            acc
        });
    Ok(num_safe.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!("4", process(input)?);
        Ok(())
    }
}
