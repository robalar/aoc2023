use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    let answer: usize = input
        .lines()
        .map(Row::from)
        .map(|r| possible_arrangements(r.springs, &r.criteria))
        .sum();

    dbg!(answer);
}

fn possible_arrangements(springs: Vec<Spring>, criteria: &[usize]) -> usize {
    match springs
        .iter()
        .find_position(|s| matches!(s, Spring::Unknown))
    {
        None => {
            if is_possible(&springs, &criteria) {
                1
            } else {
                0
            }
        }
        Some((i, _)) => {
            // Recurse down into the branches trying each path for the first unknown
            let mut with_operational = springs.clone();
            with_operational[i] = Spring::Operational;

            let mut with_damanged = springs.clone();
            with_damanged[i] = Spring::Damaged;

            possible_arrangements(with_operational, criteria)
                + possible_arrangements(with_damanged, criteria)
        }
    }
}

fn is_possible(springs: &[Spring], criteria: &[usize]) -> bool {
    let mut damanged_groups = vec![];
    let mut current_count = 0;
    for spring in springs {
        match spring {
            Spring::Damaged => {
                current_count += 1;
            }
            _ => {
                if current_count != 0 {
                    damanged_groups.push(current_count);
                }
                current_count = 0;
            }
        }
    }

    if current_count != 0 {
        damanged_groups.push(current_count);
    }

    // dbg!(&springs, &damanged_groups, criteria);

    damanged_groups == criteria
}

#[derive(Debug)]
struct Row {
    springs: Vec<Spring>,
    criteria: Vec<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

impl From<char> for Spring {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Operational,
            '#' => Self::Damaged,
            '?' => Self::Unknown,
            _ => panic!("Unknown spring type"),
        }
    }
}

impl From<&str> for Row {
    fn from(value: &str) -> Self {
        let (springs, groups) = value.split_once(' ').expect("could not split at ' '");
        Self {
            springs: springs.chars().map(Spring::from).collect(),
            criteria: groups.split(",").map(|s| s.parse().unwrap()).collect(),
        }
    }
}
