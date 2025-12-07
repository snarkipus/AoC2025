// Input: comma-separated list of ranges, each range is
// "start-stop" Example: "11-22,95-115,998-1012"
//
// A "repeating pattern" is a number where a sequence of
// digits repeats at least twice.
//
// Examples:
//     11-22:  "1|1", "2|2" -> [11, 22]
//     95-115: "9|9", "1|1|1" -> [99, 111]
//     998-1012: "9|9|9", "10|10" -> [999, 1010]
//     1188511880-1188511890: "11885|11885" -> [1188511885]
//     222220-222224: "222|222" -> [222222]
//     1698522-1698528: no repeating sequences -> []
//     446443-446449: "446|446" -> [446446]
//     38593856-38593862: "3859|3859" -> [38593859]
//     565653-565659: "565|565" -> [565565]
//     824824821-824824827: "824|824|824" -> [824824824]
//     2121212118-2121212124: "21|21|21|21|21" ->
// [2121212121]
//
// Returns: Sum of all matching numbers across all ranges

#[tracing::instrument(skip_all)]
pub fn process(input: &str) -> miette::Result<String> {
    let mut sum = 0u64;
    let ranges = extract_ranges(input)?;

    for (start, stop) in ranges {
        for number in start..=stop {
            let s = number.to_string();
            let len = s.len();

            'intervals: for interval in 1..=len / 2 {
                if len % interval != 0 {
                    // Not a clean division
                    continue;
                }

                let substring = &s[..interval];
                let repeats = len / interval;
                let mut is_match = true;

                for i in 1..repeats {
                    if &s[i * interval..(i + 1) * interval]
                        != substring
                    {
                        is_match = false;
                        break;
                    }
                }

                if is_match {
                    sum += number;
                    break 'intervals;
                }
            }
        }
    }

    Ok(sum.to_string())
}

#[tracing::instrument]
pub fn extract_ranges(
    input: &str,
) -> miette::Result<Vec<(u64, u64)>> {
    let mut ranges = Vec::<(u64, u64)>::new();

    for range_str in input.trim().split(',') {
        let (start_str, stop_str) = range_str
            .trim()
            .split_once('-')
            .ok_or_else(|| {
                miette::miette!(
                    "invalid range: {}",
                    range_str
                )
            })?;
        let start: u64 =
            start_str.parse().map_err(|e| {
                miette::miette!(
                    "invalid start value: {}",
                    e
                )
            })?;
        let stop: u64 = stop_str.parse().map_err(|e| {
            miette::miette!("invalid stop value: {}", e)
        })?;

        ranges.push((start, stop));
    }

    Ok(ranges)
}

#[cfg(test)]
mod tests {
    use tracing_test::traced_test;

    use super::*;

    #[traced_test]
    #[test]
    fn test_process() -> miette::Result<()> {
        let input = concat!(
            "11-22,95-115,998-1012,",
            "1188511880-1188511890,222220-222224,",
            "1698522-1698528,446443-446449,",
            "38593856-38593862,565653-565659,",
            "824824821-824824827,2121212118-2121212124"
        );
        assert_eq!("4174379265", process(input)?);
        Ok(())
    }
}
