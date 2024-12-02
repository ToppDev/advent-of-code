#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut list_left: Vec<u32> = Vec::new();
    let mut list_right: Vec<u32> = Vec::new();

    for line in input.lines() {
        let mut iter = line.split_whitespace();
        list_left.push(iter.next().unwrap().parse().unwrap());
        list_right.push(iter.next().unwrap().parse().unwrap());
    }
    list_left.sort();
    list_right.sort();
    let total = list_left
        .iter()
        .zip(list_right)
        .map(|(l, r)| l.abs_diff(r))
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
        assert_eq!("11", process(input)?);
        Ok(())
    }
}
