#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    todo!("day-03 - part 2");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        todo!("haven't built test yet");
        #[allow(unused)]
        let input = "";
        assert_eq!("", process(input)?);
        Ok(())
    }
}
