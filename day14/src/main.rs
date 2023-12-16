use std::{
    collections::{hash_map::DefaultHasher, HashMap, HashSet},
    fmt::Display,
    hash::{Hash, Hasher},
};

fn main() {
    let input = include_str!("input.txt");
    let mut platform = Platform::from(input);

    let mut prev_hashes = HashMap::new();
    let mut loads = HashMap::new();

    for i in 0usize.. {
        let new_hash = platform.calculate_hash();
        if let Some(prev_i) = prev_hashes.get(&new_hash) {
            // Have encountered this state before, so we must be in a loop
            let cycle_length = i - prev_i;
            // Need to account for the 'lead in' to the cycle (prev_i)
            let final_i = ((1000000000 - prev_i) % cycle_length) + prev_i;
            let final_load = loads.get(&final_i).expect("could not get final_load");
            dbg!(final_load);
            break;
        } else {
            prev_hashes.insert(new_hash, i);
            loads.insert(i, platform.total_load());
        }

        platform.spin_cycle();
    }
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

    fn tilt_west(&mut self) {
        let mut westmost_row = vec![0usize; self.height];
        for j in 0..self.width {
            for i in 0..self.height {
                if let Some(r) = self.grid.get(&(i, j)) {
                    match r {
                        Rock::Round => {
                            self.grid.remove(&(i, j));
                            self.grid.insert((i, westmost_row[i]), Rock::Round);
                            westmost_row[i] += 1;
                        }
                        Rock::Square => westmost_row[i] = j + 1,
                    }
                }
            }
        }
    }

    fn tilt_south(&mut self) {
        let mut southmost_row = vec![self.height - 1; self.width];
        for i in (0..self.height).rev() {
            for j in 0..self.width {
                if let Some(r) = self.grid.get(&(i, j)) {
                    match r {
                        Rock::Round => {
                            self.grid.remove(&(i, j));
                            self.grid.insert((southmost_row[j], j), Rock::Round);
                            southmost_row[j] = southmost_row[j].saturating_sub(1);
                        }
                        Rock::Square => southmost_row[j] = i.saturating_sub(1),
                    }
                }
            }
        }
    }

    fn tilt_east(&mut self) {
        let mut eastmost_row = vec![self.width - 1; self.height];
        for j in (0..self.width).rev() {
            for i in 0..self.height {
                if let Some(r) = self.grid.get(&(i, j)) {
                    match r {
                        Rock::Round => {
                            self.grid.remove(&(i, j));
                            self.grid.insert((i, eastmost_row[i]), Rock::Round);
                            eastmost_row[i] = eastmost_row[i].saturating_sub(1);
                        }
                        Rock::Square => eastmost_row[i] = j.saturating_sub(1),
                    }
                }
            }
        }
    }

    fn spin_cycle(&mut self) {
        self.tilt_north();
        self.tilt_west();
        self.tilt_south();
        self.tilt_east();
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

    fn calculate_hash(&self) -> u64 {
        let mut s = DefaultHasher::new();
        self.hash(&mut s);
        s.finish()
    }
}

impl Hash for Platform {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let mut coords = self.grid.keys().collect::<Vec<_>>();
        coords.sort();
        coords.hash(state);
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
