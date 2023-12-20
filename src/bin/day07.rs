use anyhow::{anyhow, Error, Result};
use nom::{
    bytes::complete::take,
    character::complete::{digit1, newline, space1},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl TryFrom<char> for Card {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '2' => Ok(Card::Two),
            '3' => Ok(Card::Three),
            '4' => Ok(Card::Four),
            '5' => Ok(Card::Five),
            '6' => Ok(Card::Six),
            '7' => Ok(Card::Seven),
            '8' => Ok(Card::Eight),
            '9' => Ok(Card::Nine),
            'T' => Ok(Card::Ten),
            'J' => Ok(Card::Jack),
            'Q' => Ok(Card::Queen),
            'K' => Ok(Card::King),
            'A' => Ok(Card::Ace),
            _ => Err(anyhow!("Cannot convert {:?} into Card", value)),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Hand {
    cards: [Card; 5],
    bid: usize,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.get_type(), self.cards).cmp(&(other.get_type(), other.cards))
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Hand {
    fn get_type(&self) -> HandType {
        let mut card_counts = [0usize; Card::Ace as usize + 1];
        for card in &self.cards {
            card_counts[*card as usize] += 1;
        }
        let num_jokers = std::mem::replace(&mut card_counts[Card::Joker as usize], 0);

        card_counts.sort_by(|a, b| b.cmp(a));
        card_counts[0] += num_jokers;

        match (card_counts[0], card_counts[1]) {
            (5, _) => HandType::FiveOfAKind,
            (4, _) => HandType::FourOfAKind,
            (3, 2) => HandType::FullHouse,
            (3, _) => HandType::ThreeOfAKind,
            (2, 2) => HandType::TwoPair,
            (2, _) => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }
}
fn parse_usize(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |s: &str| s.parse::<usize>())(input)
}

fn parse_cards(input: &str) -> IResult<&str, [Card; 5]> {
    map_res(take(5usize), |cards: &str| {
        cards
            .chars()
            .map(Card::try_from)
            .collect::<Result<Vec<Card>>>()
            .map(|cards| cards.try_into().unwrap())
    })(input)
}

fn parse_hands(input: &str) -> IResult<&str, Vec<Hand>> {
    separated_list1(
        newline,
        map(
            separated_pair(parse_cards, space1, parse_usize),
            |(cards, bid)| Hand { cards, bid },
        ),
    )(input)
}

fn part_a(hands: &mut [Hand]) -> usize {
    hands.sort();
    hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + (i + 1) * hand.bid)
}

fn part_b(hands: &mut [Hand]) -> usize {
    for hand in hands.iter_mut() {
        for card in &mut hand.cards {
            if *card == Card::Jack {
                *card = Card::Joker
            }
        }
    }

    part_a(hands)
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("res/input07.txt")?;
    let mut hands = parse_hands(&input)
        .map_err(|e| anyhow!("Error parsing input: {:?}", e))?
        .1;

    let part_a = part_a(&mut hands);
    println!("Day 07, part A: {}", part_a);

    let part_b = part_b(&mut hands);
    println!("Day 07, part B: {}", part_b);
    Ok(())
}
