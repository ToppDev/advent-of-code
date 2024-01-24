use itertools::Itertools;

pub fn process(input: &str) -> String {
    input
        .lines()
        .map(|s| s.chars().filter(|c| c.is_digit(10)).collect())
        .map(|s: String| format!("{}{}", s.chars().next().unwrap(), s.chars().last().unwrap()))
        .map(|s| s.parse::<i32>().unwrap())
        .sum::<i32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!(process(input), "142");
    }
}
