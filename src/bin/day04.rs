use anyhow::{anyhow, Result};
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, newline, space1},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::{preceded, separated_pair, tuple},
    IResult,
};

struct Card {
    count: usize,
    winning_numbers: Vec<usize>,
    numbers: Vec<usize>,
}

impl Card {
    fn num_matching(&self) -> usize {
        self.numbers
            .iter()
            .filter(|num| self.winning_numbers.contains(num))
            .count()
    }

    fn score(&self) -> usize {
        match self.num_matching() {
            0 => 0,
            num => 2usize.pow(num as u32 - 1),
        }
    }
}

fn parse_usize(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |s: &str| s.parse::<usize>())(input)
}

fn parse_input(input: &str) -> IResult<&str, Vec<Card>> {
    separated_list1(
        newline,
        preceded(
            tuple((tag("Card"), space1, digit1, tag(":"), space1)),
            map(
                separated_pair(
                    separated_list1(space1, parse_usize),
                    tuple((space1, tag("|"), space1)),
                    separated_list1(space1, parse_usize),
                ),
                |(winning_numbers, numbers)| Card {
                    count: 1,
                    winning_numbers,
                    numbers,
                },
            ),
        ),
    )(input)
}

fn part_a(cards: &[Card]) -> usize {
    cards.iter().map(Card::score).sum()
}

fn part_b(mut cards: Vec<Card>) -> usize {
    for i in 0..cards.len() {
        for j in (i + 1)..=(i + cards[i].num_matching().min(cards.len() - 1)) {
            cards[j].count += cards[i].count;
        }
    }
    cards.iter().map(|card| card.count).sum()
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("res/input04.txt")?;
    let cards = parse_input(&input)
        .map_err(|e| anyhow!("Error parsing input: {:?}", e))?
        .1;

    let part_a = part_a(&cards);
    println!("Day 04, part A: {}", part_a);

    let part_b = part_b(cards);
    println!("Day 04, part B: {}", part_b);
    Ok(())
}
