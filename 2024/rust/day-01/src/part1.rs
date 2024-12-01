use std::iter::zip;

use nom::{
    character::complete::{digit1, newline, space0},
    combinator::opt,
    multi::many1,
    sequence::{separated_pair, terminated},
    IResult,
};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (mut left_list, mut right_list) = create_lists(input);
    left_list.sort();
    right_list.sort();
    let list = zip(left_list, right_list);
    let res: u32 = list.map(|v| v.0.abs_diff(v.1)).sum();
    return Ok(format!("{}", res));
}

pub fn create_lists(input: &str) -> (Vec<u32>, Vec<u32>) {
    // Use `terminated` to ensure each line ends with a newline
    let line_parser = terminated(parse_line, opt(newline));

    // Use `many1` to parse multiple lines
    let (_, results) = many1(line_parser)(input).unwrap();
    // Split the results into two separate vectors
    let (vec1, vec2): (Vec<u32>, Vec<u32>) = results.into_iter().unzip();

    (vec1, vec2)
}

fn parse_line(input: &str) -> IResult<&str, (u32, u32)> {
    // Use separated_pair to parse two numbers separated by spaces
    let (remaining, (num1, num2)) = separated_pair(
        digit1, // First number
        space0, // Zero or more spaces
        digit1, // Second number
    )(input)?;

    // Convert the parsed numbers (as &str) into u32
    let num1 = num1.parse::<u32>().unwrap();
    let num2 = num2.parse::<u32>().unwrap();

    Ok((remaining, (num1, num2)))
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
        assert_eq!("11", process(input)?);
        Ok(())
    }
}
