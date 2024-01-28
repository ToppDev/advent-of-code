use itertools::Itertools;

pub fn process(input: &str) -> String {
    input.lines().map(process_line).sum::<u32>().to_string()
}

fn process_line(line: &str) -> u32 {
    let (_, sets) = line.split_at(line.find(':').unwrap());
    // let game: u32 = game[game.find(' ').unwrap() + 1..].parse().unwrap();
    let (r, g, b): (Vec<_>, Vec<_>, Vec<_>) = sets[1..].split(';').map(process_set).multiunzip();

    r.iter().max().unwrap() * g.iter().max().unwrap() * b.iter().max().unwrap()
}

fn process_set(set: &str) -> (u32, u32, u32) {
    set.split(',')
        .map(|set| {
            let mut sp = set.trim_start().split(' ');
            (
                sp.next().unwrap().parse::<u32>().unwrap(),
                sp.last().unwrap().trim_start(),
            )
        })
        .fold((0, 0, 0), |acc, x| {
            (
                if x.1 == "red" { acc.0 + x.0 } else { acc.0 },
                if x.1 == "green" { acc.1 + x.0 } else { acc.1 },
                if x.1 == "blue" { acc.2 + x.0 } else { acc.2 },
            )
        })
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn test_process() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(process(input), "2286");
    }

    #[rstest]
    #[case("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", 48)]
    #[case("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue", 12)]
    #[case(
        "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
        1560
    )]
    #[case(
        "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
        630
    )]
    #[case("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", 36)]
    fn line_test(#[case] line: &str, #[case] expected: u32) {
        assert_eq!(process_line(line), expected)
    }
}
