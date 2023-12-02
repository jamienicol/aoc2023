use anyhow::{anyhow, Result};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, newline},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::{delimited, separated_pair, tuple},
    IResult,
};

#[derive(Debug)]
struct Set {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<Set>,
}

fn parse_u32(input: &str) -> IResult<&str, u32> {
    map_res(digit1, |s: &str| s.parse::<u32>())(input)
}

fn parse_set(input: &str) -> IResult<&str, Set> {
    map(
        separated_list1(
            tag(", "),
            separated_pair(
                parse_u32,
                tag(" "),
                alt((tag("red"), tag("green"), tag("blue"))),
            ),
        ),
        |cubes: Vec<(u32, &str)>| {
            let mut set = Set {
                red: 0,
                green: 0,
                blue: 0,
            };
            for (num, color) in cubes {
                match color {
                    "red" => set.red += num,
                    "green" => set.green += num,
                    "blue" => set.blue += num,
                    _ => unreachable!(),
                }
            }
            set
        },
    )(input)
}

fn parse_games(input: &str) -> IResult<&str, Vec<Game>> {
    separated_list1(
        newline,
        map(
            tuple((
                delimited(tag("Game "), parse_u32, tag(": ")),
                separated_list1(tag("; "), parse_set),
            )),
            |(id, sets)| Game { id, sets },
        ),
    )(input)
}

fn part_a(games: &[Game]) -> u32 {
    games
        .iter()
        .filter_map(|game| {
            game.sets
                .iter()
                .all(|set| set.red <= 12 && set.green <= 13 && set.blue <= 14)
                .then_some(game.id)
        })
        .sum()
}

fn part_b(games: &[Game]) -> u32 {
    games
        .iter()
        .map(|game| {
            let min = game.sets.iter().fold(
                Set {
                    red: 0,
                    green: 0,
                    blue: 0,
                },
                |mut acc, set| {
                    acc.red = acc.red.max(set.red);
                    acc.green = acc.green.max(set.green);
                    acc.blue = acc.blue.max(set.blue);
                    acc
                },
            );
            min.red * min.green * min.blue
        })
        .sum()
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("res/input02.txt")?;
    let games = parse_games(&input)
        .map_err(|e| anyhow!("Error parsing input: {:?}", e))?
        .1;

    let part_a = part_a(&games);
    println!("Day 02, part A: {}", part_a);

    let part_b = part_b(&games);
    println!("Day 02, part B: {}", part_b);
    Ok(())
}
