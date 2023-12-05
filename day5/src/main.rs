use nom::{
    bytes::complete::tag,
    character::complete::one_of,
    combinator::{map_res, recognize},
    multi::{many1, separated_list0},
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};

fn main() {
    let input = include_str!("input.txt");

    let almanac = parse_input(input);

    let maps = almanac.maps();

    let answer = almanac
        .seeds
        .clone()
        .into_iter()
        .map(|s| maps.into_iter().fold(s, |acc, map| map.get(acc)))
        .min()
        .unwrap();

    dbg!(answer);
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<usize>,
    seed_to_soil: RangeMap,
    soil_to_fertilizer: RangeMap,
    fertilizer_to_water: RangeMap,
    water_to_light: RangeMap,
    light_to_temperature: RangeMap,
    temperature_to_humidity: RangeMap,
    humidity_to_location: RangeMap,
}

impl Almanac {
    fn maps(&self) -> [&RangeMap; 7] {
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
    ranges: Vec<Range>,
}

impl RangeMap {
    fn get(&self, v: usize) -> usize {
        self.ranges.iter().find_map(|r| r.get(v)).unwrap_or(v)
    }
}

#[derive(Debug)]
struct Range {
    source_start: usize,
    dest_start: usize,
    len: usize,
}

impl Range {
    fn get(&self, v: usize) -> Option<usize> {
        if v >= self.source_start && v < self.source_start + self.len {
            let dist = v - self.source_start;
            Some(self.dest_start + dist)
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

fn seeds(input: &str) -> IResult<&str, Vec<usize>> {
    preceded(tag("seeds: "), separated_list0(tag(" "), integer))(input)
}

fn integer(input: &str) -> IResult<&str, usize> {
    map_res(recognize(many1(one_of("0123456789"))), |out: &str| {
        usize::from_str_radix(out, 10)
    })(input)
}

fn range_map(input: &str) -> IResult<&str, RangeMap> {
    let (input, ranges) = separated_list0(tag("\n"), range)(input)?;

    Ok((input, RangeMap { ranges }))
}

fn range(input: &str) -> IResult<&str, Range> {
    let (input, (dest_start, source_start, len)) = tuple((
        terminated(integer, tag(" ")),
        terminated(integer, tag(" ")),
        integer,
    ))(input)?;

    Ok((
        input,
        Range {
            source_start,
            dest_start,
            len,
        },
    ))
}
