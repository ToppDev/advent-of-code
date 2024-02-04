use itertools::Itertools;
use regex::Regex;

pub fn process(input: &str) -> String {
    let chars_per_line = input.lines().next().unwrap().len();
    let fill_line = (0..chars_per_line).map(|_| ".").collect::<String>();
    let input = fill_line.clone() + "\n" + input + &fill_line;

    let reg_num = Regex::new(r"([0-9]+)").unwrap();
    let reg_sym = Regex::new(r"[^0-9\.]").unwrap();
    input
        .lines()
        .tuple_windows()
        .map(|(l1, l2, l3)| {
            reg_num
                .find_iter(l2)
                .map(|x| (x.start(), x.end()))
                .map(|(start, end)| {
                    let sym_start = if start == 0 { start } else { start - 1 };
                    let sym_end = if end == chars_per_line { end } else { end + 1 };
                    if reg_sym.is_match(&l1[sym_start..sym_end])
                        || reg_sym.is_match(&l2[sym_start..sym_end])
                        || reg_sym.is_match(&l3[sym_start..sym_end])
                    {
                        l2[start..end].parse::<u32>().unwrap()
                    } else {
                        0
                    }
                })
                .sum::<u32>()
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_process() {
        let input = indoc! {r#"
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..
        "#};
        assert_eq!(process(input), "4361");
    }
}
