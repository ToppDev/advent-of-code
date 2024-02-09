pub fn process(input: &str) -> String {
    todo!("{{crate_name}} - part 2")
}

fn process_line(line: &str) -> u32 {
    todo!("{{crate_name}} - part 2 process_line")
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use rstest::rstest;

    use super::*;

    #[test]
    fn test_process() {
        todo!("Test data missing");
        let input = indoc! {r#"
        "#};

        assert_eq!(process(input), "");
    }

    #[rstest]
    #[case("", 0)]
    fn line_test(#[case] line: &str, #[case] expected: u32) {
        assert_eq!(process_line(line), expected)
    }
}
