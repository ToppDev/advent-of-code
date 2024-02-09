use std::ops::{Shl, Shr};

use nom::{
    bytes::complete::tag, character::complete::space1, character::complete::u32,
    multi::separated_list0, IResult,
};

pub fn process(input: &str) -> String {
    input.lines().map(process_line).sum::<u32>().to_string()
}

fn process_line(line: &str) -> u32 {
    let line = parse_line(line).unwrap().1;

    let num_winning_found = line
        .winning_nums
        .iter()
        .filter(|x| line.numbers.contains(x))
        .count() as u32;

    1u32.shl(num_winning_found).shr(1)
}

struct Line {
    winning_nums: Vec<u32>,
    numbers: Vec<u32>,
}

fn parse_line(input: &str) -> IResult<&str, Line> {
    let (remainder, _) = space1(&input[4..])?; // "Card"
    let (remainder, _) = u32(remainder)?;
    let (remainder, _) = tag(":")(remainder)?;
    let (remainder, _) = space1(remainder)?;
    let (remainder, winning_nums) = separated_list0(space1, u32)(remainder)?;
    let (remainder, _) = space1(remainder)?;
    let (remainder, _) = tag("|")(remainder)?;
    let (remainder, _) = space1(remainder)?;
    let (remainder, numbers) = separated_list0(space1, u32)(remainder)?;

    Ok((
        remainder,
        Line {
            winning_nums,
            numbers,
        },
    ))
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use rstest::rstest;

    use super::*;

    #[test]
    fn test_process() {
        let input = indoc! {r#"
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        "#};
        assert_eq!(process(input), "13");
    }

    #[rstest]
    #[case("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53", 8)]
    #[case("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19", 2)]
    #[case("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1", 2)]
    #[case("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83", 1)]
    #[case("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36", 0)]
    #[case("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11", 0)]
    fn line_test(#[case] line: &str, #[case] expected: u32) {
        assert_eq!(process_line(line), expected)
    }
}
