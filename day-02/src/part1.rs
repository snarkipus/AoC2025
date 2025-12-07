// Input: comma-separated list of ranges, each range is
// "start-stop" Example: "11-22,95-115,998-1012"
//
// A "repeating pattern" is a number where the first half of
// digits equals the second half. Only numbers with even
// digit lengths can match (e.g., 11, 1010, 123123).
//
// Examples:
//     11-22:  "1|1", "2|2" -> [11, 22]
//     95-115: "9|9" -> [99]
//     998-1012: "10|10" -> [1010]
//     1188511880-1188511890: "11885|11885" -> [1188511885]
//     222220-222224: "222|222" -> [222222]
//     1698522-1698528: (all odd length) -> []
//     446443-446449: "446|446" -> [446446]
//     38593856-38593862: "3859|3859" -> [38593859]
//
// Returns: Sum of all matching numbers across all ranges

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut sum = 0u64;

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

        for number in start..=stop {
            let s = number.to_string();
            let len = s.len();

            // Skip odd-length numbers (can't have repeating
            // halves)
            if len % 2 != 0 {
                continue;
            }

            let mid = len / 2;
            let (first_half, second_half) = s.split_at(mid);

            if first_half == second_half {
                sum += number;
            }
        }
    }

    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = concat!(
            "11-22,95-115,998-1012,",
            "1188511880-1188511890,222220-222224,",
            "1698522-1698528,446443-446449,",
            "38593856-38593862,565653-565659,",
            "824824821-824824827,2121212118-2121212124"
        );
        assert_eq!("1227775554", process(input)?);
        Ok(())
    }
}
