use std::ops::Range;

use nom::{
    bytes::complete::tag,
    character::complete::one_of,
    combinator::{map_res, recognize},
    multi::{many1, separated_list0},
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    IResult,
};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

fn main() {
    let input = include_str!("example.txt");

    let almanac = parse_input(input);

    let maps = almanac.maps();

    dbg!(&almanac.seeds);

    let answer = almanac
        .seeds
        .clone()
        .into_par_iter()
        .map(|sr| {
            sr.into_par_iter()
                .map(|s| {
                    maps.into_iter().fold(s, |acc, maps| {
                        maps.iter().find_map(|r| r.get(acc)).unwrap_or(acc)
                    })
                })
                .min()
                .expect("no minimum for range")
        })
        .min()
        .expect("no overall minimum");

    dbg!(answer);
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<Range<usize>>,
    seed_to_soil: Vec<RangeMap>,
    soil_to_fertilizer: Vec<RangeMap>,
    fertilizer_to_water: Vec<RangeMap>,
    water_to_light: Vec<RangeMap>,
    light_to_temperature: Vec<RangeMap>,
    temperature_to_humidity: Vec<RangeMap>,
    humidity_to_location: Vec<RangeMap>,
}

impl Almanac {
    fn maps(&self) -> [&Vec<RangeMap>; 7] {
        [
            &self.seed_to_soil,
            &self.soil_to_fertilizer,
            &self.fertilizer_to_water,
            &self.water_to_light,
            &self.light_to_temperature,
            &self.temperature_to_humidity,
            &self.humidity_to_location,
        ]
    }
}

#[derive(Debug)]
struct RangeMap {
    source: Range<usize>,
    dest: Range<usize>,
}

impl RangeMap {
    fn get(&self, v: usize) -> Option<usize> {
        if self.source.contains(&v) {
            let dist = v - self.source.start;
            Some(self.dest.start + dist)
        } else {
            None
        }
    }
}

fn parse_input(input: &str) -> Almanac {
    let (
        _input,
        (
            seeds,
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        ),
    ) = tuple((
        terminated(seeds, tag("\n\n")),
        delimited(tag("seed-to-soil map:\n"), range_map, tag("\n\n")),
        delimited(tag("soil-to-fertilizer map:\n"), range_map, tag("\n\n")),
        delimited(tag("fertilizer-to-water map:\n"), range_map, tag("\n\n")),
        delimited(tag("water-to-light map:\n"), range_map, tag("\n\n")),
        delimited(tag("light-to-temperature map:\n"), range_map, tag("\n\n")),
        delimited(
            tag("temperature-to-humidity map:\n"),
            range_map,
            tag("\n\n"),
        ),
        preceded(tag("humidity-to-location map:\n"), range_map),
    ))(input)
    .expect("could not parse input");

    Almanac {
        seeds,
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    }
}

fn seeds(input: &str) -> IResult<&str, Vec<Range<usize>>> {
    preceded(tag("seeds: "), separated_list0(tag(" "), seed_range))(input)
}

fn seed_range(input: &str) -> IResult<&str, Range<usize>> {
    separated_pair(integer, tag(" "), integer)(input)
        .map(|(s, (start, len))| (s, start..start + len))
}

fn integer(input: &str) -> IResult<&str, usize> {
    map_res(recognize(many1(one_of("0123456789"))), |out: &str| {
        usize::from_str_radix(out, 10)
    })(input)
}

fn range_map(input: &str) -> IResult<&str, Vec<RangeMap>> {
    separated_list0(tag("\n"), range)(input)
}

fn range(input: &str) -> IResult<&str, RangeMap> {
    let (input, (dest_start, source_start, len)) = tuple((
        terminated(integer, tag(" ")),
        terminated(integer, tag(" ")),
        integer,
    ))(input)?;

    Ok((
        input,
        RangeMap {
            source: source_start..source_start + len,
            dest: dest_start..dest_start + len,
        },
    ))
}
