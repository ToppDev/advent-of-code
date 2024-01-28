#[derive(Debug, PartialEq, Eq)]
struct Cubes {
    r: u32,
    g: u32,
    b: u32,
}
const NEEDED: Cubes = Cubes {
    r: 12,
    g: 13,
    b: 14,
};

pub fn process(input: &str) -> String {
    input.lines().map(process_line).sum::<u32>().to_string()
}

fn process_line(line: &str) -> u32 {
    let (game, sets) = line.split_at(line.find(':').unwrap());
    let game: u32 = game[game.find(' ').unwrap() + 1..].parse().unwrap();
    if sets[1..].split(';').all(|set| {
        let c = process_set(set);
        c.r <= NEEDED.r && c.g <= NEEDED.g && c.b <= NEEDED.b
    }) {
        game
    } else {
        0
    }
}

fn process_set(set: &str) -> Cubes {
    set.split(',')
        .map(|set| {
            let mut sp = set.trim_start().split(' ');
            (
                sp.next().unwrap().parse::<u32>().unwrap(),
                sp.last().unwrap().trim_start(),
            )
        })
        .fold(Cubes { r: 0, g: 0, b: 0 }, |acc, x| Cubes {
            r: if x.1 == "red" { acc.r + x.0 } else { acc.r },
            g: if x.1 == "green" { acc.g + x.0 } else { acc.g },
            b: if x.1 == "blue" { acc.b + x.0 } else { acc.b },
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
        assert_eq!(process(input), "8");
    }

    #[rstest]
    #[case(" 3 blue, 4 red", Cubes{ r: 4, g: 0, b: 3 })]
    #[case("3 blue, 4 red", Cubes{ r: 4, g: 0, b: 3 })]
    #[case("1 red, 2 green, 6 blue", Cubes{ r: 1, g: 2, b: 6 })]
    #[case("2 green", Cubes{ r: 0, g: 2, b: 0 })]
    #[case("1 blue, 2 green", Cubes{ r: 0, g: 2, b: 1 })]
    #[case("3 green, 4 blue, 1 red", Cubes{ r: 1, g: 3, b: 4 })]
    #[case("1 green, 1 blue", Cubes{ r: 0, g: 1, b: 1 })]
    #[case("8 green, 6 blue, 20 red", Cubes{ r: 20, g: 8, b: 6 })]
    #[case("5 blue, 4 red, 13 green", Cubes{ r: 4, g: 13, b: 5 })]
    #[case("5 green, 1 red", Cubes{ r: 1, g: 5, b: 0 })]
    #[case("1 green, 3 red, 6 blue", Cubes{ r: 3, g: 1, b: 6 })]
    #[case("3 green, 6 red", Cubes{ r: 6, g: 3, b: 0 })]
    #[case("3 green, 15 blue, 14 red", Cubes{ r: 14, g: 3, b: 15 })]
    #[case("6 red, 1 blue, 3 green", Cubes{ r: 6, g: 3, b: 1 })]
    #[case("2 blue, 1 red, 2 green", Cubes{ r: 1, g: 2, b: 2 })]
    fn set_test(#[case] set: &str, #[case] expected: Cubes) {
        assert_eq!(process_set(set), expected)
    }
}
