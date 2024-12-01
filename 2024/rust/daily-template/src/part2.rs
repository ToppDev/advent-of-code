#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    todo!("{{crate_name}} - part 2")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        todo!("Test not yet written");
        let input = "";
        assert_eq!("", process(input)?);
        Ok(())
    }
}
