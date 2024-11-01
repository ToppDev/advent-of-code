use std::collections::HashMap;

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut lines = input.lines();
    let instructions = lines.next().unwrap();
    lines.next(); // empty line

    let nodes: HashMap<_, _> = lines
        .map(|line| {
            dbg!(line);
            (line, line)
        })
        .collect();

    Ok("".to_string())
}

fn process_line(_line: &str) -> u32 {
    todo!("day_08 - part 1 process_line")
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
