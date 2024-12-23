use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    todo!("{{crate_name}} - part 2")
}

fn process_line(_line: &str) -> u32 {
    todo!("{{crate_name}} - part 2 process_line")
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        todo!("Test not yet written");
        let input = "";

        assert_eq!(process(input)?, "");
        Ok(())
    }

    #[rstest]
    #[case("", 0)]
    fn line_test(#[case] line: &str, #[case] expected: u32) {
        assert_eq!(process_line(line), expected)
    }
}
