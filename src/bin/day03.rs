use anyhow::Result;

struct Part {
    x: usize,
    y: usize,
}

struct Number {
    num: u32,
    digits: usize,
    x: usize,
    y: usize,
}

impl Number {
    fn adjacent_to(&self, part: &Part) -> bool {
        part.x as isize >= self.x as isize - 1
            && part.x as isize <= self.x as isize + self.digits as isize
            && part.y as isize >= self.y as isize - 1
            && part.y as isize <= self.y as isize + 1
    }
}

fn parse_input(input: &str) -> Result<(Vec<Part>, Vec<Number>)> {
    let mut parts = Vec::new();
    let mut numbers = Vec::new();

    for (y, line) in input.lines().enumerate() {
        let mut x = 0;
        while x < line.chars().count() {
            match line.chars().nth(x).unwrap() {
                '.' => {}
                c if c.is_ascii_digit() => {
                    let mut num_end = x;
                    while let Some(c) = line.chars().nth(num_end) {
                        if c.is_ascii_digit() {
                            num_end += 1;
                        } else {
                            break;
                        }
                    }
                    let num = line[x..num_end].parse().unwrap();
                    numbers.push(Number {
                        num,
                        digits: num_end - x,
                        x,
                        y,
                    });
                    x = num_end - 1;
                }
                _ => parts.push(Part { x, y }),
            }
            x += 1
        }
    }

    Ok((parts, numbers))
}

fn part_a(parts: &[Part], numbers: &[Number]) -> u32 {
    numbers
        .iter()
        .filter_map(|number| {
            if parts.iter().any(|part| number.adjacent_to(part)) {
                Some(number.num)
            } else {
                None
            }
        })
        .sum()
}

fn part_b(parts: &[Part], numbers: &[Number]) -> u32 {
    parts
        .iter()
        .filter_map(|part| {
            let adjacent_nums = numbers
                .iter()
                .filter_map(|number| number.adjacent_to(part).then_some(number.num));
            if adjacent_nums.clone().count() == 2 {
                Some(adjacent_nums.product::<u32>())
            } else {
                None
            }
        })
        .sum()
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("res/input03.txt")?;
    let (parts, numbers) = parse_input(&input)?;

    let part_a = part_a(&parts, &numbers);
    println!("Day 03, part A: {}", part_a);

    let part_b = part_b(&parts, &numbers);
    println!("Day 03, part B: {}", part_b);
    Ok(())
}
