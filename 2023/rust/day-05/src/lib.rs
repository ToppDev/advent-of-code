pub mod custom_error;

pub mod part1;
pub mod part2;

use nom::{
    bytes::complete::take_till,
    character::complete::{self, newline, space1},
    multi::separated_list1,
    sequence::{pair, tuple},
    IResult, Parser,
};
use nom_supreme::ParserExt;
use tracing::info;

#[derive(Debug)]
struct Entry {
    src: u64,
    dest: u64,
    amount: u64,
}

#[derive(Debug)]
struct Map(Vec<Entry>);

impl Map {
    fn calc_dest(&self, input: u64) -> u64 {
        let entry = self
            .0
            .iter()
            .find(|e| e.src <= input && input < e.src + e.amount);
        match entry {
            Some(e) => e.dest + (input - e.src),
            None => input,
        }
    }
}

#[derive(Debug)]
struct MapCollection(Vec<Map>);

impl MapCollection {
    fn map_to_dest(&self, input: u64) -> u64 {
        self.0.iter().fold(input, |input, map| map.calc_dest(input))
    }
}

#[tracing::instrument(skip(input))]
fn entry(input: &str) -> IResult<&str, Entry> {
    let (input, (dest, src, amount)) = tuple((
        complete::u64,
        complete::u64.preceded_by(space1),
        complete::u64.preceded_by(space1),
    ))(input)?;
    Ok((input, Entry { src, dest, amount }))
}

#[tracing::instrument(skip(input), fields(map_name = input.lines().next().unwrap().split(' ').next().unwrap()))]
fn map(input: &str) -> IResult<&str, Map> {
    let (input, entries) = separated_list1(newline, entry)
        .preceded_by(take_till(char::is_numeric))
        .parse(input)?;
    info!(?entries);
    Ok((input, Map(entries)))
}

#[tracing::instrument(skip(input))]
fn maps(input: &str) -> IResult<&str, MapCollection> {
    let (input, collection) = separated_list1(pair(newline, newline), map)(input)?;
    // info!(?collection);
    Ok((input, MapCollection(collection)))
}
