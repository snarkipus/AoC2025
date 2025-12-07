#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    todo!("day-03 - part 2");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore = "not implemented yet"]
    #[test]
    fn test_process() -> miette::Result<()> {
        todo!("haven't built test yet");
        #[allow(unused)]
        let input = "";
        assert_eq!("", process(input)?);
        Ok(())
    }
}
