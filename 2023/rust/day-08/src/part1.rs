use core::panic;
use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::alpha1,
    sequence::{separated_pair, terminated},
    IResult,
};

use crate::custom_error::AocError;

#[derive(Debug)]
struct Node<'a> {
    left: &'a str,
    right: &'a str,
}

fn node(input: &str) -> IResult<&str, (&str, Node)> {
    let (input, node) = terminated(alpha1, tag(" = ("))(input)?;
    let (input, (left, right)) =
        separated_pair(alpha1, tag(", "), alpha1)(input)?;

    Ok((input, (node, Node { left, right })))
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut lines = input.lines();
    let instructions = lines.next().unwrap();
    lines.next(); // empty line

    let nodes: HashMap<_, _> =
        lines.map(|line| node(line).unwrap().1).collect();

    let mut current_node = "AAA";
    let steps = instructions
        .chars()
        .cycle()
        .take_while(|c| {
            current_node = match c {
                'L' => nodes[current_node].left,
                'R' => nodes[current_node].right,
                v => panic!("Can only be 'L' or 'R'. Got: {v}"),
            };

            current_node != "ZZZ"
        })
        .count()
        + 1;

    Ok(steps.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_1() -> miette::Result<()> {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

        assert_eq!(process(input)?, "2");
        Ok(())
    }

    #[test]
    fn test_process_2() -> miette::Result<()> {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

        assert_eq!(process(input)?, "6");
        Ok(())
    }
}
