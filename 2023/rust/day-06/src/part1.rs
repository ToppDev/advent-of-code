use itertools::Itertools;

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let data = input
        .lines()
        .map(|l| {
            let mut s = l.split_whitespace();
            s.next();
            s.collect_vec()
        })
        .collect_vec();

    Ok(format!(
        "{}",
        (0..data[0].len())
            .map(|i| {
                process_race(
                    data[0][i].parse::<u32>().unwrap(),
                    data[1][i].parse::<u32>().unwrap(),
                )
            })
            .product::<u32>()
    ))
}

fn process_race(time: u32, distance: u32) -> u32 {
    (0..=time)
        .filter(|t| {
            let remaining = time - t;
            let traveling = remaining * t;
            traveling > distance
        })
        .count() as u32
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Time:      7  15   30
Distance:  9  40  200";

        assert_eq!(process(input)?, "288");
        Ok(())
    }

    #[rstest]
    #[case(7, 9, 4)]
    #[case(15, 40, 8)]
    #[case(30, 200, 9)]
    fn line_test(
        #[case] time: u32,
        #[case] distance: u32,
        #[case] expected: u32,
    ) {
        assert_eq!(process_race(time, distance), expected)
    }
}