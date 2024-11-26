use core::panic;
use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::alphanumeric1,
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
    let (input, name) = terminated(alphanumeric1, tag(" = ("))(input)?;
    let (input, (left, right)) = separated_pair(alphanumeric1, tag(", "), alphanumeric1)(input)?;

    Ok((input, (name, Node { left, right })))
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut lines = input.lines();
    let instructions = lines.next().unwrap();
    lines.next(); // empty line

    let nodes: HashMap<_, _> = lines.map(|line| node(line).unwrap().1).collect();

    let mut starting_nodes: Vec<_> = nodes
        .iter()
        .filter(|x| x.0.ends_with('A'))
        .map(|x| *x.0)
        .collect();
    println!("Starting nodes: {starting_nodes:?}");

    // Searching till all starting nodes have a 'Z' at the end takes forever
    // The input is hover arranged in a special way, so that it
    // forms cycles (after some instructions it comes back to the start)
    //
    // With this the solution can be calculated by determining the cycle lengths
    // and then calculating the least comman multiple (lcm) of the results.
    let results = starting_nodes
        .iter_mut()
        .map(|current_node| {
            instructions
                .chars()
                .cycle()
                .take_while(|c| {
                    *current_node = match c {
                        'L' => nodes[current_node].left,
                        'R' => nodes[current_node].right,
                        v => panic!("Can only be 'L' or 'R'. Got: {v}"),
                    };

                    !current_node.ends_with('Z')
                })
                .count()
                + 1
        })
        .collect::<Vec<usize>>();
    println!("Cycles: {results:?}");

    Ok(lcm(&results).to_string())
}

pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

        assert_eq!(process(input)?, "6");
        Ok(())
    }
}
