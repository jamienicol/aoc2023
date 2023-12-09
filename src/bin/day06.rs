use anyhow::{anyhow, Result};
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, newline, space1},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::{preceded, separated_pair, tuple},
    IResult,
};

#[derive(Debug)]
struct Race {
    time: usize,
    distance: usize,
}

fn parse_usize(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |s: &str| s.parse::<usize>())(input)
}

fn parse_races(input: &str) -> IResult<&str, Vec<Race>> {
    map(
        separated_pair(
            preceded(
                tuple((tag("Time:"), space1)),
                separated_list1(space1, parse_usize),
            ),
            newline,
            preceded(
                tuple((tag("Distance:"), space1)),
                separated_list1(space1, parse_usize),
            ),
        ),
        |(times, distances)| {
            times
                .into_iter()
                .zip(distances)
                .map(|(time, distance)| Race { time, distance })
                .collect()
        },
    )(input)
}

fn part_a(races: &[Race]) -> usize {
    races
        .iter()
        .map(|race| {
            (0..=race.time)
                .filter(|speed| (race.time - speed) * speed > race.distance)
                .count()
        })
        .product()
}

fn next_pow10(x: usize) -> usize {
    let x = x as f64;
    10usize.pow(x.log10().ceil() as u32)
}

fn fix_kerning(races: &[Race]) -> Race {
    races.iter().fold(
        Race {
            time: 0,
            distance: 0,
        },
        |mut acc, race| {
            acc.time = acc.time * next_pow10(race.time) + race.time;
            acc.distance = acc.distance * next_pow10(race.distance) + race.distance;
            acc
        },
    )
}

fn part_b(races: &[Race]) -> usize {
    part_a(&[fix_kerning(races)])
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("res/input06.txt")?;
    let races = parse_races(&input)
        .map_err(|e| anyhow!("Error parsing input: {:?}", e))?
        .1;

    let part_a = part_a(&races);
    println!("Day 06, part A: {}", part_a);

    let part_b = part_b(&races);
    println!("Day 06, part B: {}", part_b);
    Ok(())
}
