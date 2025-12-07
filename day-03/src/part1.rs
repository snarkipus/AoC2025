use miette::miette;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Battery {
    pub joltage: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bank {
    pub batteries: Vec<Battery>,
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut banks = Vec::<Bank>::new();

    for line in input.lines() {
        let bank = line
            .trim()
            .chars()
            .map(|joltage| -> miette::Result<Battery> {
                let joltage = joltage
                    .to_digit(10)
                    .ok_or_else(|| {
                        miette!(
                            "invalid joltage character: {}",
                            joltage
                        )
                    })?;
                Ok(Battery { joltage })
            })
            .collect::<miette::Result<Vec<Battery>>>()?;
        banks.push(Bank { batteries: bank });
    }

    // For each bank:
    //  - find the highest value from left to right:
    //    max_left
    //  - find the highest value from right to left:
    //    max_right (not including the max_left position)
    //  - concatenate the two results into a single value:
    //    peak_joltage
    let sum: u32 = banks
        .iter()
        .map(|bank| -> miette::Result<u32> {
            let batteries = &bank.batteries;
            let search_end = batteries.len().saturating_sub(1);

            let (peak_pos, max_left) = batteries[..search_end]
                .iter()
                .enumerate()
                .max_by_key(|(i, b)| (b.joltage, std::cmp::Reverse(*i)))
                .map(|(i, b)| (i, b.joltage))
                .ok_or_else(|| {
                    miette!("No left-side peak found (bank might be too small)")
                })?;

            let max_right = batteries[peak_pos + 1..]
                .iter()
                .map(|b| b.joltage)
                .max()
                .ok_or_else(|| {
                    miette!("No right-side value found after peak at index {}", peak_pos)
                })?;

            let peak_joltage = max_left * 10 + max_right;
            Ok(peak_joltage)
        })
        .sum::<miette::Result<u32>>()?;

    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tracing_test::traced_test]
    #[test]
    fn test_process() -> miette::Result<()> {
        let input = concat!(
            "987654321111111\n",
            "811111111111119\n",
            "234234234234278\n",
            "818181911112111"
        );
        assert_eq!("357", process(input)?);
        Ok(())
    }
}
