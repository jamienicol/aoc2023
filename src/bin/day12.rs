use std::collections::HashMap;

use anyhow::{anyhow, Result};
use nom::{
    character::complete::{char as achar, digit1, newline, one_of, space1},
    combinator::{map, map_res},
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

impl Spring {
    fn maybe_operational(&self) -> bool {
        *self == Spring::Operational || *self == Spring::Unknown
    }

    fn maybe_damaged(&self) -> bool {
        *self == Spring::Damaged || *self == Spring::Unknown
    }
}

struct Row {
    springs: Vec<Spring>,
    damaged: Vec<usize>,
}

fn parse_usize(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |s: &str| s.parse::<usize>())(input)
}

fn parse_spring(input: &str) -> IResult<&str, Spring> {
    map(one_of(".#?"), |c| match c {
        '.' => Spring::Operational,
        '#' => Spring::Damaged,
        '?' => Spring::Unknown,
        _ => unreachable!(),
    })(input)
}

fn parse_input(input: &str) -> IResult<&str, Vec<Row>> {
    separated_list1(
        newline,
        map(
            separated_pair(
                many1(parse_spring),
                space1,
                separated_list1(achar(','), parse_usize),
            ),
            |(springs, damaged)| Row { springs, damaged },
        ),
    )(input)
}

fn num_arrangements<'a>(
    springs: &'a [Spring],
    damaged_lens: &'a [usize],
    cache: &mut HashMap<(&'a [Spring], &'a [usize]), usize>,
) -> usize {
    if damaged_lens.is_empty() {
        if springs.iter().all(Spring::maybe_operational) {
            return 1;
        } else {
            return 0;
        }
    }

    // Move forward to the next (possibly) damaged spring
    let i = match springs.iter().position(Spring::maybe_damaged) {
        Some(i) => i,
        None => return 0,
    };
    let springs = &springs[i..];

    if let Some(res) = cache.get(&(springs, damaged_lens)) {
        return *res;
    }

    let mut count = 0;
    // If the next spring is damaged (assuming it is if it's unknown) then check whether the next
    // run of damaged springs can fit here
    if springs[0].maybe_damaged() {
        let (run_len, damaged_lens) = damaged_lens.split_first().unwrap();
        if springs.len() >= *run_len {
            let run_springs = &springs[..*run_len];
            let mut rem_springs = &springs[*run_len..];
            if run_springs.iter().all(Spring::maybe_damaged)
                && (rem_springs.is_empty() || rem_springs[0].maybe_operational())
            {
                // The damaged run fits. So recursively find the number of matching arrangements
                // for the remaining damaged runs
                if !rem_springs.is_empty() {
                    rem_springs = &rem_springs[1..];
                }
                count += num_arrangements(rem_springs, damaged_lens, cache);
            }
        }
    }

    // Add to that the number of matching arrangements if we assume the unknown is operational.
    if springs[0] == Spring::Unknown {
        count += num_arrangements(&springs[1..], damaged_lens, cache);
    }

    cache.insert((springs, damaged_lens), count);

    count
}

fn solve(rows: &[Row]) -> usize {
    let mut cache = HashMap::new();
    rows.iter()
        .map(|row| num_arrangements(&row.springs, &row.damaged, &mut cache))
        .sum()
}

fn unfold(rows: &mut [Row], n: usize) {
    for row in rows.iter_mut() {
        let orig_springs = 0..row.springs.len();
        let orig_damaged = 0..row.damaged.len();
        for _ in 0..(n - 1) {
            row.springs.push(Spring::Unknown);
            row.springs.extend_from_within(orig_springs.clone());
            row.damaged.extend_from_within(orig_damaged.clone());
        }
    }
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("res/input12.txt")?;
    let mut rows = parse_input(&input)
        .map_err(|e| anyhow!("Error parsing input: {:?}", e))?
        .1;

    let part_a = solve(&rows);
    println!("Day 12, part A: {}", part_a);

    unfold(&mut rows, 5);

    let part_b = solve(&rows);
    println!("Day 12, part B: {}", part_b);
    Ok(())
}
