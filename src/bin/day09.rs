use anyhow::{anyhow, Result};
use itertools::Itertools;
use nom::{
    character::complete::{char as achar, digit1, newline, space1},
    combinator::{map_res, opt, recognize},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

fn parse_isize(input: &str) -> IResult<&str, isize> {
    map_res(recognize(tuple((opt(achar('-')), digit1))), |s: &str| {
        s.parse::<isize>()
    })(input)
}

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<isize>>> {
    separated_list1(newline, separated_list1(space1, parse_isize))(input)
}

fn part_a(sensors: &[Vec<Vec<isize>>]) -> isize {
    sensors
        .iter()
        .map(|diffs| {
            diffs
                .iter()
                .rev()
                .skip(1)
                .fold(0, |acc, diff| diff.last().unwrap() + acc)
        })
        .sum()
}

fn part_b(sensors: &[Vec<Vec<isize>>]) -> isize {
    sensors
        .iter()
        .map(|diffs| {
            diffs
                .iter()
                .rev()
                .skip(1)
                .fold(0, |acc, diff| diff.first().unwrap() - acc)
        })
        .sum()
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("res/input09.txt")?;
    let readings = parse_input(&input)
        .map_err(|e| anyhow!("Error parsing input: {:?}", e))?
        .1;

    // For each sensor, recursively make a list of the differences between each reading
    // until all the differences are zero.
    let sensors = readings
        .into_iter()
        .map(|reading| {
            let mut diffs = vec![reading];
            while diffs.last().unwrap().iter().any(|n| *n != 0) {
                let prev = diffs.last().unwrap();
                let new = prev
                    .iter()
                    .tuple_windows()
                    .map(|(a, b)| b - a)
                    .collect_vec();
                diffs.push(new)
            }
            diffs
        })
        .collect::<Vec<Vec<Vec<isize>>>>();

    let part_a = part_a(&sensors);
    println!("Day 09, part A: {}", part_a);

    let part_b = part_b(&sensors);
    println!("Day 09, part B: {}", part_b);
    Ok(())
}
