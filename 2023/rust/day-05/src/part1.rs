use nom::{
    character::complete::{self, newline, space1},
    multi::separated_list1,
    sequence::pair,
    IResult, Parser,
};
use nom_supreme::{tag::complete::tag, ParserExt};
use tracing::info;

use crate::{custom_error::AocError, maps};

#[tracing::instrument()]
fn seeds(input: &str) -> IResult<&str, Vec<u64>> {
    tag("seeds: ")
        .precedes(separated_list1(space1, complete::u64))
        .parse(input)
}

#[tracing::instrument(skip(input), fields(input_first_line = input.lines().next().unwrap()))]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let (remaining, seeds) = seeds.parse(input).unwrap();
    info!(?seeds);
    let collection = maps
        .preceded_by(pair(newline, newline))
        .parse(remaining)
        .unwrap()
        .1;
    Ok(seeds
        .iter()
        .map(|s| collection.map_to_dest(*s))
        .min()
        .unwrap()
        .to_string())
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use rstest::{fixture, rstest};

    use crate::{Entry, Map, MapCollection};

    use super::*;

    #[test_log::test]
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

        assert_eq!(process(input)?, "35");
        Ok(())
    }

    #[fixture]
    fn seed_to_soil() -> Map {
        Map(vec![
            Entry {
                dest: 50,
                src: 98,
                amount: 2,
            },
            Entry {
                dest: 52,
                src: 50,
                amount: 48,
            },
        ])
    }

    #[rstest]
    #[case(98, 50)]
    #[case(99, 51)]
    #[case(49, 49)]
    #[case(100, 100)]
    #[case(50, 52)]
    #[case(97, 99)]
    fn map_test(seed_to_soil: Map, #[case] input: u64, #[case] expected: u64) {
        assert_eq!(seed_to_soil.calc_dest(input), expected)
    }

    #[fixture]
    fn seed_to_soil_to_fertilizer() -> MapCollection {
        MapCollection(vec![
            Map(vec![
                Entry {
                    dest: 50,
                    src: 98,
                    amount: 2,
                },
                Entry {
                    dest: 52,
                    src: 50,
                    amount: 48,
                },
            ]),
            Map(vec![
                Entry {
                    dest: 0,
                    src: 15,
                    amount: 37,
                },
                Entry {
                    dest: 37,
                    src: 52,
                    amount: 2,
                },
                Entry {
                    dest: 39,
                    src: 0,
                    amount: 15,
                },
            ]),
        ])
    }

    #[rstest]
    #[case(98, 35)]
    #[case(50, 37)]
    fn map_collection_test(
        seed_to_soil_to_fertilizer: MapCollection,
        #[case] input: u64,
        #[case] expected: u64,
    ) {
        assert_eq!(seed_to_soil_to_fertilizer.map_to_dest(input), expected)
    }
}
