use itertools::Itertools;

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let (time, distance) = input
        .lines()
        .map(|l| {
            let (_, l) = l.split_once(':').unwrap();
            l.replace(' ', "").parse::<u64>().unwrap()
        })
        .collect_tuple()
        .unwrap();

    Ok(format!("{}", process_race(time, distance)))
}

fn process_race(time: u64, distance: u64) -> u64 {
    (0..=time)
        .filter(|t| {
            let remaining = time - t;
            let traveling = remaining * t;
            traveling > distance
        })
        .count() as u64
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Time:      7  15   30
Distance:  9  40  200";

        assert_eq!(process(input)?, "71503");
        Ok(())
    }

    #[test]
    fn test_process_race() -> miette::Result<()> {
        let time = 71530;
        let distance = 940200;

        assert_eq!(process_race(time, distance), 71503);
        Ok(())
    }
}
