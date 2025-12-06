/// Dial starts at position 50 on a circular dial numbered
/// 0-99. Each line is a rotation: `Lnn` or `Rnn`.
/// - `L`: rotate left (decreasing numbers) nn clicks.
/// - `R`: rotate right (increasing numbers) nn clicks.
/// Wrapping around: left from 0 goes to 99, right from 99
/// goes to 0.
///
/// The password is the number of rotations that end with
/// the dial pointing at 0.
#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut position: u8 = 50;
    let mut count: usize = 0;

    for line in input.lines() {
        let (dir, dist_str) = line.split_at(1);
        let dist: usize = dist_str.parse()
            .map_err(|e| miette::miette!("Failed to parse distance '{}': {} in line '{}'", dist_str, e, line))?;
        let dist_mod = dist % 100;
        let new_pos: u8 = match dir {
            "L" => ((position as usize + 100 - delta) % 100) as u8,
            "R" => ((position as usize + delta) % 100) as u8,
            _ => {
                return Err(miette::miette!(
                    "Invalid direction '{}': must be L or R",
                    dir
                ));
            }
        };
        position = new_pos;
        if position == 0 {
            count += 1;
        }
    }

    Ok(count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = [
            "L68", "L30", "R48", "L5", "R60", "L55", "L1",
            "L99", "R14", "L82",
        ]
        .join("\n");
        assert_eq!(process(&input)?, "3");
        Ok(())
    }
}
