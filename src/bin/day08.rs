use std::collections::HashMap;

use anyhow::{anyhow, Result};
use itertools::Itertools;
use nom::{
    bytes::complete::{tag, take},
    character::complete::{newline, one_of},
    combinator::map,
    multi::{fold_many1, many1},
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Left,
    Right,
}

type Node = [char; 3];
type Map = HashMap<Node, (Node, Node)>;

fn parse_directions(input: &str) -> IResult<&str, Vec<Direction>> {
    many1(map(one_of("LR"), |dir| match dir {
        'L' => Direction::Left,
        'R' => Direction::Right,
        _ => unreachable!(),
    }))(input)
}

fn parse_node_name(input: &str) -> IResult<&str, Node> {
    map(take(3usize), |name: &str| {
        let (a, b, c) = name.chars().collect_tuple().unwrap();
        [a, b, c]
    })(input)
}

fn parse_node(input: &str) -> IResult<&str, (Node, Node, Node)> {
    tuple((
        terminated(parse_node_name, tag(" = (")),
        terminated(parse_node_name, tag(", ")),
        terminated(parse_node_name, tag(")")),
    ))(input)
}

fn parse_input(input: &str) -> IResult<&str, (Vec<Direction>, Map)> {
    separated_pair(
        parse_directions,
        newline,
        fold_many1(
            preceded(newline, parse_node),
            Map::new,
            |mut acc, (start, left, right)| {
                acc.insert(start, (left, right));
                acc
            },
        ),
    )(input)
}

fn find_end(
    start: Node,
    directions: &[Direction],
    map: &Map,
    is_end: impl Fn(Node) -> bool,
) -> usize {
    let mut pos = start;
    let mut num_steps = 0;
    let mut dir_iter = directions.iter().cycle();
    while !is_end(pos) {
        num_steps += 1;
        let next = map[&pos];
        pos = match dir_iter.next().unwrap() {
            Direction::Left => next.0,
            Direction::Right => next.1,
        };
    }
    num_steps
}

fn part_a(directions: &[Direction], map: &Map) -> usize {
    find_end(['A', 'A', 'A'], directions, map, |pos| {
        pos == ['Z', 'Z', 'Z']
    })
}

fn part_b(directions: &[Direction], map: &Map) -> usize {
    let positions = map
        .keys()
        .filter(|[_, _, c]| *c == 'A')
        .cloned()
        .collect::<Vec<Node>>();

    // Find the number of steps it takes to reach a destination from each starting position
    let num_steps = positions
        .iter()
        .map(|pos| find_end(*pos, directions, map, |pos| pos[2] == 'Z'))
        .collect_vec();

    // Find the lowest multiple of each of the numbers of steps
    let most_steps = *num_steps.iter().max().unwrap();
    (most_steps..)
        .step_by(most_steps)
        .find(|i| num_steps.iter().all(|n| i % n == 0))
        .unwrap()
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("res/input08.txt")?;
    let (directions, map) = parse_input(&input)
        .map_err(|e| anyhow!("Error parsing input: {:?}", e))?
        .1;

    let part_a = part_a(&directions, &map);
    println!("Day 08, part A: {}", part_a);

    let part_b = part_b(&directions, &map);
    println!("Day 08, part B: {}", part_b);
    Ok(())
}
