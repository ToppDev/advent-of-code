use std::collections::HashMap;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut list_left: Vec<u32> = Vec::new();
    let mut list_right: HashMap<u32, u32> = HashMap::new();

    for line in input.lines() {
        let mut iter = line.split_whitespace();
        let (left, right) = (
            iter.next().unwrap().parse().unwrap(),
            iter.next().unwrap().parse().unwrap(),
        );
        list_left.push(left);
        list_right.entry(right).and_modify(|e| *e += 1).or_insert(1);
    }
    let total = list_left
        .iter()
        .map(|l| l * list_right.get(l).unwrap_or(&0))
        .sum::<u32>();
    Ok(total.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!("31", process(input)?);
        Ok(())
    }
}
