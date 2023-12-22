use std::collections::HashMap;

use anyhow::{bail, Context, Result};
use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Pipe: u32 {
        const NORTH = 0b00000001;
        const SOUTH = 0b00000010;
        const EAST  = 0b00000100;
        const WEST  = 0b00001000;
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Pos {
    x: isize,
    y: isize,
}

impl Pos {
    fn new(x: isize, y: isize) -> Self {
        Pos { x, y }
    }
}

struct Map {
    width: usize,
    height: usize,
    pipes: Vec<Pipe>,
}

impl Map {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pipes: vec![Pipe::empty(); width * height],
        }
    }

    fn is_pos_valid(&self, pos: Pos) -> bool {
        pos.x >= 0 && pos.x < self.width as isize && pos.y >= 0 && pos.y < self.height as isize
    }

    fn pipe(&self, pos: Pos) -> Pipe {
        assert!(self.is_pos_valid(pos));
        self.pipes[pos.y as usize * self.width + pos.x as usize]
    }

    fn pipe_mut(&mut self, pos: Pos) -> &mut Pipe {
        assert!(self.is_pos_valid(pos));
        &mut self.pipes[pos.y as usize * self.width + pos.x as usize]
    }

    fn is_connected(&self, a: Pos, b: Pos) -> bool {
        match (b.x - a.x, b.y - a.y) {
            (0, 1) => self.pipe(a).contains(Pipe::SOUTH) && self.pipe(b).contains(Pipe::NORTH),
            (0, -1) => self.pipe(a).contains(Pipe::NORTH) && self.pipe(b).contains(Pipe::SOUTH),
            (1, 0) => self.pipe(a).contains(Pipe::EAST) && self.pipe(b).contains(Pipe::WEST),
            (-1, 0) => self.pipe(a).contains(Pipe::WEST) && self.pipe(b).contains(Pipe::EAST),
            _ => false,
        }
    }

    // Iterator yielding the direction and position of all neighbours that are connected.
    fn connected_neighbours(&self, pos: Pos) -> impl Iterator<Item = (Pipe, Pos)> + '_ {
        [
            (Pipe::WEST, Pos::new(pos.x - 1, pos.y)),
            (Pipe::EAST, Pos::new(pos.x + 1, pos.y)),
            (Pipe::NORTH, Pos::new(pos.x, pos.y - 1)),
            (Pipe::SOUTH, Pos::new(pos.x, pos.y + 1)),
        ]
        .into_iter()
        .filter(|(_direction, pos)| self.is_pos_valid(*pos))
        .filter(move |(_dir, neighbour_pos)| self.is_connected(pos, *neighbour_pos))
    }
}

fn parse_input(input: &str) -> Result<(Map, Pos)> {
    let height = input.lines().count();
    let width = input.lines().next().context("Empty input")?.chars().count();

    let mut start_pos = None;
    let mut map = Map::new(width, height);
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            map.pipes[y * width + x] = match c {
                '|' => Pipe::NORTH | Pipe::SOUTH,
                '-' => Pipe::EAST | Pipe::WEST,
                'L' => Pipe::NORTH | Pipe::EAST,
                'J' => Pipe::NORTH | Pipe::WEST,
                '7' => Pipe::SOUTH | Pipe::WEST,
                'F' => Pipe::SOUTH | Pipe::EAST,
                '.' => Pipe::empty(),
                'S' => {
                    if start_pos.is_some() {
                        bail!("Duplicate start positions found");
                    }
                    start_pos = Some(Pos::new(x as isize, y as isize));
                    // Initially set start position as connected in all directions. This will be
                    // fixed up later in fix_start().
                    Pipe::all()
                }
                _ => bail!("Unexpected character {:?} found", c),
            }
        }
    }

    Ok((map, start_pos.context("No start position found")?))
}

fn fix_start(map: &mut Map, start_pos: Pos) {
    assert!(map.pipe(start_pos) == Pipe::all());
    let start_pipe = map
        .connected_neighbours(start_pos)
        .fold(Pipe::empty(), |acc, (dir, _)| acc | dir);
    *map.pipe_mut(start_pos) = start_pipe;
}

fn find_loop(map: &Map, start_pos: Pos) -> HashMap<Pos, usize> {
    let mut open: HashMap<Pos, usize> = HashMap::new();
    let mut closed: HashMap<Pos, usize> = HashMap::new();

    open.insert(start_pos, 0);

    // Perform a breadth-first search from the start position. When we encounter a position that's
    // already on our open list then we have completed the loop at the furthest away distance.
    while let Some((current_pos, current_distance)) = open
        .iter()
        .min_by_key(|(_pos, distance)| *distance)
        .map(|(pos, distance)| (*pos, *distance))
    {
        open.remove(&current_pos);
        closed.insert(current_pos, current_distance);

        for neighbour_pos in map
            .connected_neighbours(current_pos)
            .map(|(_, neighbour_pos)| neighbour_pos)
            .filter(|neighbour_pos| !closed.contains_key(neighbour_pos))
        {
            if let Some(distance) = open.get(&neighbour_pos) {
                // Add this node to our closed set then that gives us the loop.
                closed.insert(neighbour_pos, *distance);
                return closed;
            } else {
                open.insert(neighbour_pos, current_distance + 1);
            }
        }
    }

    unreachable!("Failed to find pipe loop");
}

fn part_a(pipe_loop: &HashMap<Pos, usize>) -> usize {
    *pipe_loop.values().max().unwrap()
}

fn part_b(pipe_loop: &HashMap<Pos, usize>, map: &Map) -> usize {
    let mut count = 0;

    // If we cross the pipe an odd number of times from the outside (y=0) then we must be inside
    // the loop. Checking only WEST (or only EAST) rather than both allows for squeezing between
    // pipes.
    for x in 0..map.width as isize {
        let mut in_loop = false;
        for y in 0..map.height as isize {
            let pos = Pos::new(x, y);
            if pipe_loop.contains_key(&pos) {
                if map.pipe(pos).contains(Pipe::WEST) {
                    in_loop = !in_loop;
                }
            } else if in_loop {
                count += 1;
            }
        }
    }

    count
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("res/input10.txt")?;
    let (mut map, start_pos) = parse_input(&input)?;

    fix_start(&mut map, start_pos);
    let pipe_loop = find_loop(&map, start_pos);

    let part_a = part_a(&pipe_loop);
    println!("Day 10, part A: {}", part_a);

    let part_b = part_b(&pipe_loop, &map);
    println!("Day 10, part B: {}", part_b);
    Ok(())
}
