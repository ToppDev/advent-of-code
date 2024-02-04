use colored::Colorize;
use itertools::Itertools;
use regex::Regex;

pub fn find_num_adjacent_to_star(line: &str, pos: usize) -> Vec<u128> {
    let chars_per_line = line.len();
    let reg_num = Regex::new(r"([0-9]+)").unwrap();
    reg_num
        .find_iter(line)
        .map(|x| (x.start(), x.end()))
        .filter_map(|(start, end)| {
            let sym_start = if start == 0 { start } else { start - 1 };
            let sym_end = if end == chars_per_line { end } else { end + 1 };
            // println!("      Search [{sym_start}..{sym_end}] for star {pos}");
            if (sym_start..sym_end).contains(&pos) {
                // println!("      Number {} found", &line[start..end]);
                line[start..end].parse().ok()
            } else {
                None
            }
        })
        .collect()
}

pub fn process(input: &str) -> String {
    let chars_per_line = input.lines().next().unwrap().len();
    let fill_line = (0..chars_per_line).map(|_| ".").collect::<String>();
    let input = fill_line.clone() + "\n" + input + &fill_line;

    let mut stars: Vec<(usize, Vec<(usize, bool, Vec<u128>, u128)>)> = Vec::new();
    let reg_star = Regex::new(r"(\*)").unwrap();
    let result = input
        .lines()
        .enumerate()
        .tuple_windows()
        .map(|(l1, l2, l3)| {
            // println!(
            //     "Checking:\n    {}\n  > {}\n    {}",
            //     l1.1.replace('*', &"*".red().to_string()),
            //     l2.1.replace('*', &"*".red().to_string()),
            //     l3.1.replace('*', &"*".red().to_string()),
            // );
            let gear_ratio = reg_star
                .find_iter(l2.1)
                .map(|x| x.start())
                .map(|star| {
                    let nums = [l1.1, l2.1, l3.1]
                        .iter()
                        .flat_map(|line| {
                            // println!("    Searching for number in line: {line}");
                            find_num_adjacent_to_star(line, star)
                        })
                        .collect::<Vec<u128>>();
                    if nums.len() == 2 {
                        let gear_ratio = nums.iter().product::<u128>();
                        // println!("  Gear ratio: {gear_ratio} {:?}", nums);
                        if let Some(chars) = stars.iter_mut().find(|(x, _)| *x == l2.0) {
                            chars.1.push((star, true, nums, gear_ratio));
                        } else {
                            stars.push((l2.0, vec![(star, true, nums, gear_ratio)]));
                        }
                        gear_ratio
                    } else {
                        if let Some(chars) = stars.iter_mut().find(|(x, _)| *x == l2.0) {
                            chars.1.push((star, false, vec![], 0));
                        } else {
                            stars.push((l2.0, vec![(star, false, vec![], 0)]));
                        }
                        0
                    }
                })
                .sum::<u128>();
            // println!("  Gear ratio: {gear_ratio} (line sum)");
            gear_ratio
        })
        .sum::<u128>()
        .to_string();

    // println!("stars: {:?}", &stars);
    let mut output: String = String::new();
    let mut y_old = 1;
    for (y, chars) in stars {
        input.lines().skip(y_old).take(y - y_old).for_each(|l| {
            output += l;
            output += "\n";
        });
        let line = input.lines().nth(y).unwrap();
        let mut x_old = 0;
        for (x, good, _, _) in &chars {
            output += &line[x_old..*x];
            if *good {
                output += &"*".green().to_string();
            } else {
                output += &"*".red().to_string();
            }
            x_old = x + 1;
        }
        output += &line[x_old..];
        let mut first = true;
        for (_, good, nums, gear_ratio) in chars {
            if good {
                if !first {
                    output += " +";
                }
                output += &format!(" {gear_ratio} {nums:?}");
                first = false;
            }
        }
        output += "\n";
        y_old = y + 1;
    }
    input.lines().skip(y_old).for_each(|l| {
        if l != fill_line {
            output += l;
            output += "\n";
        }
    });

    println!("{output}");
    result
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
        assert_eq!(process(input), "467835");
    }
}
