use std::fmt::Display;

use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");

    let grid = Grid(
        input
            .lines()
            .map(|l| l.chars().map(|c| c.into()).collect())
            .collect(),
    );

    println!("{}", grid);

    let (mut current_position, mut current_direction) =
        grid.starting_point().expect("could find starting point");
    let mut path_points = vec![];
    loop {
        path_points.push(current_position);

        let offset = current_direction.offset();

        let next_position = (
            (current_position.0 as i32 + offset.0) as usize,
            (current_position.1 as i32 + offset.1) as usize,
        );

        let next_tile = grid
            .get(next_position.0, next_position.1)
            .expect("could find next tile");

        match next_tile {
            Tile::Ground => panic!("Unclosed loop!"),
            Tile::Starting => {
                path_points.push(next_position);
                break;
            }
            Tile::Pipe(p) => {
                current_position = next_position;
                current_direction = p
                    .next_direction(&current_direction)
                    .expect("could get next direction");
            }
        }
    }

    // Pick's therorem + Shoelace sum :)
    let shoelace_sum: i32 = path_points
        .iter()
        .tuple_windows()
        .map(|(a, b)| (a.0 + b.0) as i32 * (a.1 as i32 - b.1 as i32))
        .sum();

    let answer = (shoelace_sum - path_points.len() as i32 + 4) / 2;
    dbg!(answer);
}

#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn offset(&self) -> (i32, i32) {
        match self {
            Direction::North => (-1, 0),
            Direction::East => (0, 1),
            Direction::South => (1, 0),
            Direction::West => (0, -1),
        }
    }
}

struct Grid(Vec<Vec<Tile>>);

impl Grid {
    fn starting_point(&self) -> Option<((usize, usize), Direction)> {
        let point = self.0.iter().enumerate().find_map(|(i, row)| {
            let j = row.iter().enumerate().find_map(|(j, tile)| match tile {
                Tile::Starting => Some(j),
                _ => None,
            })?;

            Some((i, j))
        })?;

        let direction = [
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ]
        .into_iter()
        .find_map(|d| {
            let offset = d.offset();

            let y = (point.0 as i32 + offset.0) as usize;
            let x = (point.1 as i32 + offset.1) as usize;

            let adjacent = self.get(y, x)?;

            match adjacent {
                Tile::Ground => None,
                Tile::Starting => None,
                Tile::Pipe(p) => p.next_direction(&d).and(Some(d)),
            }
        })?;

        Some((point, direction))
    }

    fn get(&self, y: usize, x: usize) -> Option<&Tile> {
        self.0.get(y as usize).and_then(|r| r.get(x as usize))
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();
        for row in &self.0 {
            for tile in row {
                out.push_str(&tile.to_string());
            }
            out.push('\n');
        }

        write!(f, "{}", out)
    }
}

#[derive(Debug)]
enum Tile {
    Ground,
    Starting,
    Pipe(Pipe),
}

#[derive(Debug)]
enum Pipe {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
}

impl Pipe {
    fn next_direction(&self, incoming: &Direction) -> Option<Direction> {
        match (self, incoming) {
            (Pipe::Vertical, Direction::North) => Some(Direction::North),
            (Pipe::Vertical, Direction::South) => Some(Direction::South),
            (Pipe::Horizontal, Direction::East) => Some(Direction::East),
            (Pipe::Horizontal, Direction::West) => Some(Direction::West),
            (Pipe::NorthEast, Direction::South) => Some(Direction::East),
            (Pipe::NorthEast, Direction::West) => Some(Direction::North),
            (Pipe::NorthWest, Direction::South) => Some(Direction::West),
            (Pipe::NorthWest, Direction::East) => Some(Direction::North),
            (Pipe::SouthWest, Direction::North) => Some(Direction::West),
            (Pipe::SouthWest, Direction::East) => Some(Direction::South),
            (Pipe::SouthEast, Direction::West) => Some(Direction::South),
            (Pipe::SouthEast, Direction::North) => Some(Direction::East),
            _ => None,
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let symbol = match self {
            Tile::Ground => ".",
            Tile::Starting => "S",
            Tile::Pipe(p) => match p {
                Pipe::Vertical => "║",
                Pipe::Horizontal => "═",
                Pipe::NorthEast => "╚",
                Pipe::NorthWest => "╝",
                Pipe::SouthWest => "╗",
                Pipe::SouthEast => "╔",
            },
        };

        write!(f, "{}", symbol)
    }
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '|' => Tile::Pipe(Pipe::Vertical),
            '-' => Tile::Pipe(Pipe::Horizontal),
            'L' => Tile::Pipe(Pipe::NorthEast),
            'J' => Tile::Pipe(Pipe::NorthWest),
            '7' => Tile::Pipe(Pipe::SouthWest),
            'F' => Tile::Pipe(Pipe::SouthEast),
            '.' => Tile::Ground,
            'S' => Tile::Starting,
            _ => panic!("Unknown tile {}", value),
        }
    }
}
