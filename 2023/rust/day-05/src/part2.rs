use indicatif::{ParallelProgressIterator, ProgressStyle};
use nom::{
    character::complete::{self, newline, space1},
    multi::separated_list1,
    sequence::{pair, separated_pair},
    IResult, Parser,
};
use nom_supreme::{tag::complete::tag, ParserExt};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{custom_error::AocError, maps};

#[tracing::instrument]
fn seeds(input: &str) -> IResult<&str, Vec<(u64, u64)>> {
    tag("seeds: ")
        .precedes(separated_list1(
            space1,
            separated_pair(complete::u64, space1, complete::u64),
        ))
        .parse(input)
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let (remaining, seeds) = seeds.parse(input).unwrap();
    let collection = maps
        .preceded_by(pair(newline, newline))
        .parse(remaining)
        .unwrap()
        .1;
    // let seed_count = seeds.iter().fold(0, |acc, (_, amount)| acc + amount);
    let minimum_location = seeds
        .into_par_iter()
        .flat_map(|(start, amount)| (start..start + amount))
        // .progress_count(seed_count)
        // .with_message("Calculating Locations")
        // .with_style(
        //     ProgressStyle::with_template(
        //         "[{elapsed_precise}/{duration_precise}] {wide_bar} {pos}/{len} {msg}",
        //     )
        //     .unwrap(),
        // )
        .map(|s| collection.map_to_dest(s))
        .min();
    Ok(minimum_location.unwrap().to_string())
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = indoc! {r#"
          seeds: 79 14 55 13

          seed-to-soil map:
          50 98 2
          52 50 48

          soil-to-fertilizer map:
          0 15 37
          37 52 2
          39 0 15

          fertilizer-to-water map:
          49 53 8
          0 11 42
          42 0 7
          57 7 4

          water-to-light map:
          88 18 7
          18 25 70

          light-to-temperature map:
          45 77 23
          81 45 19
          68 64 13

          temperature-to-humidity map:
          0 69 1
          1 0 69

          humidity-to-location map:
          60 56 37
          56 93 4
        "#};

        assert_eq!(process(input)?, "46");
        Ok(())
    }
}
