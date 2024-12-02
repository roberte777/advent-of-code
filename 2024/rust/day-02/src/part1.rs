use nom::{
    character::complete::{newline, space1, u32},
    combinator::opt,
    multi::{many1, separated_list1},
    sequence::terminated,
    IResult,
};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let reports = parse_input(input);
    let num_safe = reports.into_iter().map(|r| r.is_safe()).fold(0, |acc, x| {
        if x {
            return acc + 1;
        }
        acc
    });
    Ok(num_safe.to_string())
}

pub fn parse_input(input: &str) -> Vec<Report> {
    let line_parser = terminated(parse_report, opt(newline));
    let (_, reports) = many1(line_parser)(input).unwrap();
    reports
}

pub fn parse_report(input: &str) -> IResult<&str, Report> {
    let (rest, levels) = separated_list1(space1, u32)(input)?;
    Ok((rest, Report { levels }))
}

#[derive(Debug)]
pub struct Report {
    pub levels: Vec<u32>,
}

impl Report {
    pub fn is_safe(&self) -> bool {
        // check inc and dec
        let safety_type = match self.levels[0].cmp(&self.levels[1]) {
            std::cmp::Ordering::Equal => return false,
            std::cmp::Ordering::Less => SafetyType::Increasing,
            std::cmp::Ordering::Greater => SafetyType::Decreasing,
        };

        // check diff
        for i in 0..self.levels.len() {
            let curr = self.levels[i];
            match safety_type {
                SafetyType::Increasing => {
                    // if the next value isn't greater than current, not safe
                    if i + 1 < self.levels.len() && self.levels[i + 1] <= curr {
                        return false;
                    }
                }
                // if the next value isn't less than current, not safe
                SafetyType::Decreasing => {
                    if i + 1 < self.levels.len() && self.levels[i + 1] >= curr {
                        return false;
                    }
                }
            }
            if i > 0 {
                let diff = self.levels[i - 1].abs_diff(curr);
                if !(1..=3).contains(&diff) {
                    return false;
                }
            }

            if i < self.levels.len() - 1 {
                let diff = self.levels[i + 1].abs_diff(curr);
                if !(1..=3).contains(&diff) {
                    return false;
                }
            }
        }
        true
    }
    pub fn is_safe_2(&self) -> bool {
        if !Self::is_safe_inner(self.levels.clone()) {
            for index in 0..self.levels.len() {
                let mut new_report = self.levels.clone();
                new_report.remove(index);
                if Self::is_safe_inner(new_report) {
                    return true;
                } else {
                    continue;
                }
            }
            return false;
        }

        true
    }
    fn is_safe_inner(levels: Vec<u32>) -> bool {
        // check inc and dec
        let safety_type = match levels[0].cmp(&levels[1]) {
            std::cmp::Ordering::Equal => return false,
            std::cmp::Ordering::Less => SafetyType::Increasing,
            std::cmp::Ordering::Greater => SafetyType::Decreasing,
        };

        // check diff
        for i in 0..levels.len() {
            let curr = levels[i];
            match safety_type {
                SafetyType::Increasing => {
                    // if the next value isn't greater than current, not safe
                    if i + 1 < levels.len() && levels[i + 1] <= curr {
                        return false;
                    }
                }
                // if the next value isn't less than current, not safe
                SafetyType::Decreasing => {
                    if i + 1 < levels.len() && levels[i + 1] >= curr {
                        return false;
                    }
                }
            }
            if i > 0 {
                let diff = levels[i - 1].abs_diff(curr);
                if !(1..=3).contains(&diff) {
                    return false;
                }
            }

            if i < levels.len() - 1 {
                let diff = levels[i + 1].abs_diff(curr);
                if !(1..=3).contains(&diff) {
                    return false;
                }
            }
        }
        true
    }
}

pub enum SafetyType {
    Increasing,
    Decreasing,
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
        assert_eq!("2", process(input)?);
        Ok(())
    }
}
