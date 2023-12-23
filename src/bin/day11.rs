use anyhow::Result;
use itertools::Itertools;

#[derive(Debug)]
struct Pos {
    x: isize,
    y: isize,
}

fn expand(galaxies: &[Pos], amount: isize) -> Vec<Pos> {
    let (width, height) = galaxies.iter().fold((0, 0), |(max_x, max_y), galaxy| {
        (max_x.max(galaxy.x), max_y.max(galaxy.y))
    });

    let empty_rows = (0..height)
        .filter(|y| galaxies.iter().all(|galaxy| galaxy.y != *y))
        .collect_vec();
    let empty_columns = (0..width)
        .filter(|x| galaxies.iter().all(|galaxy| galaxy.x != *x))
        .collect_vec();

    galaxies
        .iter()
        .map(|galaxy| {
            let prev_empty_cols = empty_columns
                .iter()
                .filter(|empty_x| galaxy.x > **empty_x)
                .count() as isize;
            let prev_empty_rows = empty_rows
                .iter()
                .filter(|empty_y| galaxy.y > **empty_y)
                .count() as isize;
            Pos {
                x: galaxy.x + prev_empty_cols * amount,
                y: galaxy.y + prev_empty_rows * amount,
            }
        })
        .collect_vec()
}

fn parse_input(input: &str) -> Vec<Pos> {
    let mut galaxies = Vec::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                galaxies.push(Pos {
                    x: x as isize,
                    y: y as isize,
                });
            }
        }
    }

    galaxies
}

fn distances(galaxies: &[Pos]) -> usize {
    galaxies
        .iter()
        .tuple_combinations()
        .map(|(a, b)| ((b.x - a.x).abs() + (b.y - a.y).abs()) as usize)
        .sum()
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("res/input11.txt")?;
    let map = parse_input(&input);

    let map_a = expand(&map, 1);
    let part_a = distances(&map_a);
    println!("Day 11, part A: {}", part_a);

    let map_b = expand(&map, 999999);
    let part_b = distances(&map_b);
    println!("Day 11, part B: {}", part_b);
    Ok(())
}
