use anyhow::{anyhow, Result};

fn solve(input: &str) -> Result<u32> {
    let result = input.lines().try_fold(0, |acc, line| {
        let digits = line.chars().filter_map(|c| c.to_digit(10));
        let first = digits
            .clone()
            .next()
            .ok_or_else(|| anyhow!("Line does not contain any digits"))?;
        let last = digits.last().unwrap();
        Ok::<_, anyhow::Error>(acc + first * 10 + last)
    })?;

    Ok(result)
}

fn fixup_input(input: &str) -> String {
    let mut output = String::new();
    let spellings = [
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ];
    for line in input.lines() {
        for (i, c) in line.chars().enumerate() {
            for (spelling, val) in &spellings {
                if line[i..].starts_with(spelling) {
                    output.push(*val);
                } else if c == *val {
                    output.push(c)
                }
            }
        }
        output.push('\n');
    }
    output
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("res/input01.txt")?;

    let part_a = solve(&input)?;
    println!("Day 01, part A: {}", part_a);

    let fixed_input = fixup_input(&input);
    let part_b = solve(&fixed_input)?;
    println!("Day 01, part B: {}", part_b);

    Ok(())
}
