use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::one_of,
    combinator::{map_res, recognize},
    multi::{many1, separated_list1},
    sequence::{preceded, separated_pair, terminated},
    IResult, Parser,
};

fn main() {
    let input = include_str!("input.txt");

    let games = parse_records(input);

    let answer: usize = games
        .iter()
        .filter(|g| {
            g.observations.iter().all(|o| {
                o.cubes.iter().all(|c| match *c {
                    Cube::Red(count) => count <= 12,
                    Cube::Green(count) => count <= 13,
                    Cube::Blue(count) => count <= 14,
                })
            })
        })
        .map(|g| g.id)
        .sum();

    dbg!(answer);
}

#[derive(Debug)]
struct Game {
    id: usize,
    observations: Vec<Observation>,
}

#[derive(Debug)]
struct Observation {
    cubes: Vec<Cube>,
}

#[derive(Debug)]

enum Cube {
    Red(i32),
    Green(i32),
    Blue(i32),
}

fn parse_records(input: &str) -> Vec<Game> {
    let (input, games) = separated_list1(tag("\n"), game)(input).expect("parsing failed");
    assert!(input.is_empty(), "whole string was not consumed");

    games
}

fn game(input: &str) -> IResult<&str, Game> {
    let (input, id) = terminated(id, tag(": "))(input)?;
    let (input, observations) = observations(input)?;

    Ok((input, Game { id, observations }))
}

fn id(input: &str) -> IResult<&str, usize> {
    map_res(
        preceded(
            tag("Game "),
            recognize(many1(one_of("0123456789abcdefABCDEF"))),
        ),
        |out: &str| usize::from_str_radix(out, 10),
    )
    .parse(input)
}

fn observations(input: &str) -> IResult<&str, Vec<Observation>> {
    separated_list1(tag("; "), observation)(input)
}

fn observation(input: &str) -> IResult<&str, Observation> {
    let (input, cubes) = separated_list1(tag(", "), cube)(input)?;

    Ok((input, Observation { cubes }))
}

fn cube(input: &str) -> IResult<&str, Cube> {
    let (input, (count, colour)) = separated_pair(
        map_res(
            recognize(many1(one_of("0123456789abcdefABCDEF"))),
            |out: &str| i32::from_str_radix(out, 10),
        ),
        tag(" "),
        alt((tag("red"), tag("green"), tag("blue"))),
    )(input)?;

    let cube = match colour {
        "red" => Cube::Red(count),
        "green" => Cube::Green(count),
        "blue" => Cube::Blue(count),
        _ => unreachable!(),
    };

    Ok((input, cube))
}
