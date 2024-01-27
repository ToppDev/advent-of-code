pub fn process(input: &str) -> String {
    todo!("{{crate_name}} - part 1")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        todo!("Test data missing");
        let input = "";
        assert_eq!(process(input), "");
    }

    #[rstest]
    #[case("", 0)]
    fn line_test(#[case] line: &str, #[case] expected: u32) {
        assert_eq!(process_line(line), expected)
    }
}
