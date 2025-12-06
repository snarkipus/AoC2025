/// Dial starts at position 50 on a circular dial numbered
/// 0-99. Each line is a rotation: `Lnn` or `Rnn`.
/// - `L`: rotate left (decreasing numbers) nn clicks.
/// - `R`: rotate right (increasing numbers) nn clicks.
/// Wrapping around: left from 0 goes to 99, right from 99
/// goes to 0.
///
/// The password is the total number of zero crossings during rotations
/// (times the dial wraps over position 0).
#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut position: u8 = 50;
    let mut crossings: usize = 0;

    for line in input.lines() {
        let (dir, dist_str) = line.split_at(1);
        let dist: usize = dist_str.parse()
        let mut dist_to_first_zero = if dir == "R" {
            100 - p
        } else {
            p
        };
        if dir == "L" && p == 0 {
            dist_to_first_zero = 100;
        }
        let num_hits = if dist < dist_to_first_zero {
            0
        } else {
            1 + (dist - dist_to_first_zero) / 100
        };
        crossings += num_hits;
        let new_pos = if dir == "L" {
            ((p + 100 - delta) % 100) as u8
        } else if dir == "R" {
            ((p + delta) % 100) as u8
        } else {
            return Err(miette::miette!(
                "Invalid direction '{}': must be L or R",
                dir
            ));
        };
        position = new_pos;
    }

    Ok(crossings.to_string())
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
        assert_eq!(process(&input)?, "6");
        Ok(())
    }
}