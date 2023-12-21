use std::{collections::{HashMap, HashSet}, fmt::Display};

fn main() {
    let input = include_str!("example.txt");
    let grid = Grid::from(input);



    let mut seen = HashSet::new();
    grid.energised_count(
        Pose {
            position: (0, -1),
            direction: Direction::East,
        },
        0,
        &mut seen,
    );

    dbg!(seen.iter().map(|p| p.position).collect::<HashSet<_>>().len() - 1);

}

#[derive(Debug)]
struct Grid {
    map: HashMap<(i32, i32), Tile>,
    width: usize,
    height: usize,
}

impl Grid {
    fn energised_count(&self, current_pose: Pose, prev_count: usize, seen: &mut HashSet<Pose>) -> usize {
        if seen.contains(&current_pose) {
            return prev_count;
        }

        seen.insert(current_pose.clone());

        if let Some((next_position, next_tile)) = self.next_tile(&current_pose) {
            match next_tile {
                Tile::MirrorForwards => match current_pose.direction {
                    Direction::North => self.energised_count(
                        Pose {
                            position: next_position,
                            direction: Direction::East,
                        },
                        prev_count + 1,
                        seen,
                    ),
                    Direction::East => self.energised_count(
                        Pose {
                            position: next_position,
                            direction: Direction::North,
                        },
                        prev_count + 1,
                        seen,
                    ),
                    Direction::South => self.energised_count(
                        Pose {
                            position: next_position,
                            direction: Direction::West,
                        },
                        prev_count + 1,
                        seen
                    ),
                    Direction::West => self.energised_count(
                        Pose {
                            position: next_position,
                            direction: Direction::South,
                        },
                        prev_count + 1,
                        seen,
                    ),
                },
                Tile::MirrorBackwards => match current_pose.direction {
                    Direction::North => self.energised_count(
                        Pose {
                            position: next_position,
                            direction: Direction::West,
                        },
                        prev_count + 1,
                        seen,
                    ),
                    Direction::East => self.energised_count(
                        Pose {
                            position: next_position,
                            direction: Direction::South,
                        },
                        prev_count + 1,
                        seen,
                    ),
                    Direction::South => self.energised_count(
                        Pose {
                            position: next_position,
                            direction: Direction::East,
                        },
                        prev_count + 1,
                        seen,
                    ),
                    Direction::West => self.energised_count(
                        Pose {
                            position: next_position,
                            direction: Direction::North,
                        },
                        prev_count + 1,
                        seen,
                    ),
                },
                Tile::SplitterVertical => match current_pose.direction {
                    Direction::North => self.energised_count(
                        Pose {
                            position: next_position,
                            direction: Direction::North,
                        },
                        prev_count + 1,
                        seen,
                    ),
                    Direction::South => self.energised_count(
                        Pose {
                            position: next_position,
                            direction: Direction::South,
                        },
                        prev_count + 1,
                        seen,
                    ),
                    Direction::East | Direction::West => {
                        self.energised_count(
                            Pose {
                                position: next_position,
                                direction: Direction::North,
                            },
                            prev_count + 1,
                            seen
                        ) + self.energised_count(
                            Pose {
                                position: next_position,
                                direction: Direction::South,
                            },
                            prev_count + 1,
                            seen
                        )
                    }
                },
                Tile::SplitterHorizontal => match current_pose.direction {
                    Direction::West => self.energised_count(
                        Pose {
                            position: next_position,
                            direction: Direction::West,
                        },
                        prev_count + 1,
                        seen
                    ),
                    Direction::East => self.energised_count(
                        Pose {
                            position: next_position,
                            direction: Direction::East,
                        },
                        prev_count + 1,
                        seen
                    ),
                    Direction::North | Direction::South => {
                        self.energised_count(
                            Pose {
                                position: next_position,
                                direction: Direction::East,
                            },
                            prev_count + 1,
                            seen,
                        ) + self.energised_count(
                            Pose {
                                position: next_position,
                                direction: Direction::West,
                            },
                            prev_count + 1,
                            seen,
                        )
                    }
                },
                Tile::Empty => self.energised_count(Pose { position: next_position, direction: current_pose.direction }, prev_count + 1, seen)
            }
        } else {
            prev_count
        }
    }

    fn next_tile(&self, current_pose: &Pose) -> Option<((i32, i32), &Tile)> {
        let next_position = current_pose.next_position();

        if next_position.0 < 0
            || next_position.1 < 0
            || next_position.0 > (self.height as i32 - 1)
            || next_position.1 > (self.width as i32 - 1)
        {
            None
        } else {
            Some((
                next_position,
                self.map.get(&next_position).expect("couldn't find tile"),
            ))
        }
    }

    fn format_with_beam(&self, poses: &HashSet<Pose>) -> String {
        let mut positions: HashMap<_, _> = HashMap::new();
        for pose in poses {
            *positions.entry(pose.position).or_insert(0) += 1;
        }
        
        let mut string = String::with_capacity(self.width * self.height);
        for i in 0..self.height {
            for j in 0..self.width {
                let c = match self.map.get(&(i as i32, j as i32)).unwrap() {
                    Tile::MirrorForwards => '/',
                    Tile::MirrorBackwards => '\\',
                    Tile::SplitterVertical => '|',
                    Tile::SplitterHorizontal => '-',
                    Tile::Empty => {
                        match positions.get(&(i as i32, j as i32)){
                            None => '.',
                            Some(_) => '#'
                        }
                    },
                };

                string.push(c);
            }
            string.push('\n');
        }
        string
    }
}

#[derive(Debug)]
enum Tile {
    MirrorForwards,
    MirrorBackwards,
    SplitterVertical,
    SplitterHorizontal,
    Empty,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Pose {
    position: (i32, i32),
    direction: Direction,
}

impl Pose {
    fn next_position(&self) -> (i32, i32) {
        let delta = match self.direction {
            Direction::North => (-1, 0),
            Direction::East => (0, 1),
            Direction::South => (1, 0),
            Direction::West => (0, -1),
        };

        (
            self.position.0 as i32 + delta.0,
            self.position.1 as i32 + delta.1,
        )
    }
}

impl Display for Pose {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self.direction {
            Direction::North => '^',
            Direction::East => '>',
            Direction::South => '<',
            Direction::West => 'V',
        };
        write!(f, "{}", c)
    }
}

impl From<&str> for Grid {
    fn from(value: &str) -> Self {
        let width = value.find("\n").expect("could not find newline");
        let height = value.lines().count();
        let map = value
            .lines()
            .enumerate()
            .flat_map(|(i, l)| {
                l.char_indices()
                    .filter_map(move |(j, c)| Some(((i as i32, j as i32), c.into())))
            })
            .collect();
        Self { map, width, height }
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.height {
            for j in 0..self.width {
                let c = match self.map.get(&(i as i32, j as i32)).unwrap() {
                    Tile::MirrorForwards => '/',
                    Tile::MirrorBackwards => '\\',
                    Tile::SplitterVertical => '|',
                    Tile::SplitterHorizontal => '-',
                    Tile::Empty => '.',
                };

                write!(f, "{}", c)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '/' => Tile::MirrorForwards,
            '\\' => Tile::MirrorBackwards,
            '|' => Tile::SplitterVertical,
            '-' => Tile::SplitterHorizontal,
            '.' => Tile::Empty,
            _ => panic!("No tiles"),
        }
    }
}
