use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::char,
    combinator::value,
    error::{convert_error, ParseError, VerboseError},
    multi::{many0, separated_list1},
    sequence::{delimited, separated_pair},
    Err, IResult,
};

fn main() {
    let input = include_str!("input.txt");
    let map = Map::from(input);

    let inital_ghosts = map
        .network
        .keys()
        .filter(|k| k.ends_with("A"))
        .map(|n| Ghost::new(*n));

    let answer = inital_ghosts
        .map(|ghost| {
            map.directions
                .iter()
                .cycle()
                .scan((ghost, 0u64), |(ghost, count), direction| {
                    if ghost.at_terminal_node() {
                        return None;
                    }

                    let choices = map
                        .network
                        .get(ghost.current_node)
                        .expect("could find node");
                    let next_node = match direction {
                        Direction::Left => choices.0,
                        Direction::Right => choices.1,
                    };

                    ghost.current_node = next_node;
                    *count += 1;

                    Some(*count)
                })
                .last()
                .unwrap()
        })
        .fold(1, |acc, x| num::integer::lcm(acc, x));

    dbg!(answer);
}

#[derive(Debug)]
struct Ghost<'a> {
    current_node: &'a str,
}

impl<'a> Ghost<'a> {
    fn new(current_node: &'a str) -> Self {
        Ghost { current_node }
    }

    fn at_terminal_node(&self) -> bool {
        self.current_node.ends_with("Z")
    }
}

#[derive(Clone, Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct Map<'a> {
    directions: Vec<Direction>,
    network: HashMap<&'a str, (&'a str, &'a str)>,
}

impl<'a> From<&'a str> for Map<'a> {
    fn from(value: &'a str) -> Self {
        match map::<VerboseError<&str>>(value) {
            Ok((_, map)) => map,
            Err(Err::Error(e)) | Err(Err::Failure(e)) => {
                panic!("{}", convert_error(value, e));
            }
            Err(Err::Incomplete(x)) => panic!("parser incomplete: {:?}", x),
        }
    }
}

fn map<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&str, Map<'_>, E> {
    let (input, (directions, network)) =
        separated_pair(left_right_instructions, tag("\n\n"), network)(input)?;

    Ok((
        input,
        Map {
            directions,
            network,
        },
    ))
}

fn left_right_instructions<'a, E: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&str, Vec<Direction>, E> {
    many0(alt((
        value(Direction::Right, char('R')),
        value(Direction::Left, char('L')),
    )))(input)
}

fn network<'a, E: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, HashMap<&str, (&str, &str)>, E> {
    let (input, nodes) = separated_list1(char('\n'), node)(input)?;

    Ok((input, nodes.into_iter().collect()))
}

fn node<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&str, (&str, (&str, &str)), E> {
    separated_pair(
        take(3usize),
        tag(" = "),
        delimited(
            char('('),
            separated_pair(take(3usize), tag(", "), take(3usize)),
            char(')'),
        ),
    )(input)
}
