pub fn process(input: &str) -> String {
    input.lines().map(process_line).sum::<u32>().to_string()
}

fn process_line(line: &str) -> u32 {
    let mut idx = 0;
    let line_iter = std::iter::from_fn(move || {
        let line = &line[idx..];
        let result = if line.starts_with("one") {
            Some('1')
        } else if line.starts_with("two") {
            Some('2')
        } else if line.starts_with("three") {
            Some('3')
        } else if line.starts_with("four") {
            Some('4')
        } else if line.starts_with("five") {
            Some('5')
        } else if line.starts_with("six") {
            Some('6')
        } else if line.starts_with("seven") {
            Some('7')
        } else if line.starts_with("eight") {
            Some('8')
        } else if line.starts_with("nine") {
            Some('9')
        } else {
            line.chars().next()
        };
        idx += 1;
        result
    });
    let mut it = line_iter.filter_map(|c| c.to_digit(10));
    let first = it.next().expect("There should be a number");
    let last = it.last().unwrap_or(first);
    format!("{first}{last}").parse::<u32>().unwrap()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn test_process() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!(process(input), "281");
    }

    #[rstest]
    #[case("two1nine", 29)]
    #[case("eightwothree", 83)]
    #[case("abcone2threexyz", 13)]
    #[case("xtwone3four", 24)]
    #[case("4nineeightseven2", 42)]
    #[case("zoneight234", 14)]
    #[case("7pqrstsixteen", 76)]
    #[case("kvskplbpgfninesixvzkrv1fqnrjnrhvnpkpkhph27twonemv", 91)]
    #[case("zkrv1fqnrjnrhvnpkpkhph27twonemv", 11)]
    fn line_test(#[case] line: &str, #[case] expected: u32) {
        assert_eq!(expected, process_line(line))
    }
}
