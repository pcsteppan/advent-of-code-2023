use std::{collections::HashMap, fs, str::FromStr};

#[derive(Debug, Clone)]
struct Pull {
    red: i32,
    blue: i32,
    green: i32,
}

#[derive(Debug, PartialEq, Eq)]
struct PullParseError;

impl FromStr for Pull {
    type Err = PullParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result = s
            .split(",")
            .map(|n| {
                let (n, color) = n.trim().split_once(" ").unwrap();
                (color.to_string(), n.parse::<i32>().unwrap())
            })
            .into_iter();

        let hash: HashMap<String, i32> = HashMap::from_iter(result);

        Ok(Pull {
            red: hash.get("red").unwrap_or(&0).to_owned(),
            blue: hash.get("blue").unwrap_or(&0).to_owned(),
            green: hash.get("green").unwrap_or(&0).to_owned(),
        })
    }
}

#[derive(Debug, Clone)]
struct Game {
    id: i32,
    pulls: Vec<Pull>,
}

#[derive(Debug, PartialEq, Eq)]
struct GameParseError;

impl FromStr for Game {
    type Err = GameParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id_part, pulls) = s.split_once(":").unwrap();
        let id = id_part.split_once(" ").unwrap().1.parse::<i32>().unwrap();

        Ok(Game {
            id,
            pulls: pulls
                .split(";")
                .map(|p| Pull::from_str(p).unwrap())
                .collect::<Vec<_>>(),
        })
    }
}

fn main() {
    // part1();
    part2();
}

fn part1() {
    let input = fs::read_to_string("./input.txt").expect("could not open input file");
    let games = input.lines().map(|l| Game::from_str(l).unwrap());

    let color_limits = Pull {
        red: 12,
        green: 13,
        blue: 14,
    };

    let valid_games = games.filter(|g| {
        g.clone().pulls.into_iter().all(|p| {
            p.blue <= color_limits.blue
                && p.red <= color_limits.red
                && p.green <= color_limits.green
        })
    });

    let sum: i32 = valid_games.map(|g| g.id).sum();

    println!("{}", sum);
}

fn part2() {
    let input = fs::read_to_string("./input.txt").expect("could not open input file");
    let sum: i32 = input
        .lines()
        .filter_map(|l| Game::from_str(l).ok())
        .map(|g| {
            g.pulls
                .into_iter()
                .reduce(|acc, curr| Pull {
                    red: acc.red.max(curr.red),
                    blue: acc.blue.max(curr.blue),
                    green: acc.green.max(curr.green),
                })
                .unwrap()
        })
        .map(|max_pull| max_pull.blue * max_pull.red * max_pull.green)
        .sum();

    println!("{}", sum);
}
