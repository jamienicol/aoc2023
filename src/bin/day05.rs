use anyhow::{anyhow, Result};
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, newline, space1},
    combinator::{map, map_res},
    multi::{many1, separated_list0, separated_list1},
    sequence::{preceded, terminated, tuple},
    IResult,
};

#[derive(Debug)]
struct Mapping {
    dest: usize,
    source: usize,
    len: usize,
}

struct Map {
    #[allow(dead_code)]
    name: String,
    mappings: Vec<Mapping>,
}

impl Map {
    fn lookup(&self, id: usize) -> usize {
        self.mappings
            .iter()
            .find_map(|mapping| {
                (mapping.source..(mapping.source + mapping.len))
                    .contains(&id)
                    .then_some(mapping.dest + id - mapping.source)
            })
            .unwrap_or(id)
    }
}
struct Almanac {
    seeds: Vec<usize>,
    seed_to_soil_map: Map,
    soil_to_fertilizer_map: Map,
    fertilizer_to_water_map: Map,
    water_to_light_map: Map,
    light_to_temperature_map: Map,
    temperature_to_humidity_map: Map,
    humidity_to_location_map: Map,
}

fn parse_usize(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |s: &str| s.parse::<usize>())(input)
}

pub fn parse_seeds(input: &str) -> IResult<&str, Vec<usize>> {
    preceded(tag("seeds: "), separated_list1(space1, parse_usize))(input)
}

fn parse_map(name: &'static str) -> impl Fn(&str) -> IResult<&str, Map> {
    move |input: &str| {
        map(
            preceded(
                tuple((tag(name), tag(" map:"), newline)),
                separated_list0(
                    newline,
                    map(
                        tuple((
                            terminated(parse_usize, space1),
                            terminated(parse_usize, space1),
                            parse_usize,
                        )),
                        |(dest, source, len)| Mapping { dest, source, len },
                    ),
                ),
            ),
            |mappings| Map {
                name: name.to_string(),
                mappings,
            },
        )(input)
    }
}

fn parse_almanac(input: &str) -> IResult<&str, Almanac> {
    map(
        tuple((
            parse_seeds,
            preceded(many1(newline), parse_map("seed-to-soil")),
            preceded(many1(newline), parse_map("soil-to-fertilizer")),
            preceded(many1(newline), parse_map("fertilizer-to-water")),
            preceded(many1(newline), parse_map("water-to-light")),
            preceded(many1(newline), parse_map("light-to-temperature")),
            preceded(many1(newline), parse_map("temperature-to-humidity")),
            preceded(many1(newline), parse_map("humidity-to-location")),
        )),
        |(
            seeds,
            seed_to_soil_map,
            soil_to_fertilizer_map,
            fertilizer_to_water_map,
            water_to_light_map,
            light_to_temperature_map,
            temperature_to_humidity_map,
            humidity_to_location_map,
        )| Almanac {
            seeds,
            seed_to_soil_map,
            soil_to_fertilizer_map,
            fertilizer_to_water_map,
            water_to_light_map,
            light_to_temperature_map,
            temperature_to_humidity_map,
            humidity_to_location_map,
        },
    )(input)
}

fn seed_to_location(seed: usize, almanac: &Almanac) -> usize {
    let soil = almanac.seed_to_soil_map.lookup(seed);
    let fertilizer = almanac.soil_to_fertilizer_map.lookup(soil);
    let water = almanac.fertilizer_to_water_map.lookup(fertilizer);
    let light = almanac.water_to_light_map.lookup(water);
    let temperature = almanac.light_to_temperature_map.lookup(light);
    let humidity = almanac.temperature_to_humidity_map.lookup(temperature);
    almanac.humidity_to_location_map.lookup(humidity)
}

fn part_a(almanac: &Almanac) -> usize {
    almanac
        .seeds
        .iter()
        .map(|seed| seed_to_location(*seed, almanac))
        .min()
        .unwrap()
}

fn part_b(almanac: &Almanac) -> usize {
    almanac
        .seeds
        .chunks_exact(2)
        .flat_map(|seeds| {
            let start = seeds[0];
            let len = seeds[1];
            start..(start + len)
        })
        .map(|seed| seed_to_location(seed, almanac))
        .min()
        .unwrap()
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("res/input05.txt")?;
    let almanac = parse_almanac(&input)
        .map_err(|e| anyhow!("Error parsing input: {:?}", e))?
        .1;

    let part_a = part_a(&almanac);
    println!("Day 05, part A: {}", part_a);

    let part_b = part_b(&almanac);
    println!("Day 05, part B: {}", part_b);
    Ok(())
}
