#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    todo!("day-02 - part 2");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[ignore = "part 2 not implemented"]
    fn test_process() -> miette::Result<()> {
        todo!("haven't built test yet");
    }
}
