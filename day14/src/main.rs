use std::{collections::HashMap, fmt::Display};

fn main() {
    let input = include_str!("input.txt");
    let mut platform = Platform::from(input);
    platform.tilt_north();
    println!("{}", platform);
    dbg!(platform.total_load());
}

#[derive(Debug)]
struct Platform {
    grid: HashMap<(usize, usize), Rock>,
    width: usize,
    height: usize,
}

impl Platform {
    fn tilt_north(&mut self) {
        let mut lowest_row = vec![0usize; self.width];
        for i in 0..self.height {
            for j in 0..self.width {
                if let Some(r) = self.grid.get(&(i, j)) {
                    match r {
                        Rock::Round => {
                            self.grid.remove(&(i, j));
                            self.grid.insert((lowest_row[j], j), Rock::Round);
                            lowest_row[j] += 1;
                        }
                        Rock::Square => lowest_row[j] = i + 1,
                    }
                }
            }
        }
    }

    fn total_load(&self) -> usize {
        (0..self.height)
            .map(|i| {
                (self.height - i)
                    * (0..self.width)
                        .filter(|j| {
                            self.grid
                                .get(&(i, *j))
                                .map(|r| matches!(r, Rock::Round))
                                .unwrap_or(false)
                        })
                        .count()
            })
            .sum()
    }
}

impl From<&str> for Platform {
    fn from(value: &str) -> Self {
        let width = value.find("\n").expect("could not find newline");
        let height = value.lines().count();
        let grid = value
            .lines()
            .enumerate()
            .flat_map(|(i, l)| {
                l.char_indices()
                    .filter_map(move |(j, c)| Some(((i, j), c.try_into().ok()?)))
            })
            .collect();
        Self {
            grid,
            width,
            height,
        }
    }
}

impl Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.height {
            for j in 0..self.width {
                let c = match self.grid.get(&(i, j)) {
                    None => '.',
                    Some(r) => match r {
                        Rock::Round => 'O',
                        Rock::Square => '#',
                    },
                };
                write!(f, "{}", c)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[derive(Debug)]
enum Rock {
    Round,
    Square,
}

impl TryFrom<char> for Rock {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Rock::Square),
            'O' => Ok(Rock::Round),
            _ => Err("No rock"),
        }
    }
}
