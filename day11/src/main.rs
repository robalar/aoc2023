use std::fmt::Display;

use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    let mut univese = Universe::from(input);
    univese.expand();
    // println!("{}", &univese);

    let answer = univese
        .galaxies
        .iter()
        .tuple_combinations()
        .map(|(a, b)| {
            (a.0 as i64 - b.0 as i64).abs() as u64 + (a.1 as i64 - b.1 as i64).abs() as u64
        })
        .sum::<u64>();

    dbg!(answer);
}

#[derive(Debug)]
struct Universe {
    galaxies: Vec<(usize, usize)>,
    width: usize,
    height: usize,
}

impl Universe {
    fn expand(&mut self) {
        let scaling_factor = 999999;

        // Find rows without galaxies
        let rows = (0..self.height)
            .filter(|i| (0..self.width).all(|j| !self.galaxies.contains(&(*i, j))))
            .rev()
            .collect::<Vec<_>>();

        for i in rows {
            self.height += scaling_factor;
            for galaxy in self.galaxies.iter_mut() {
                if galaxy.0 > i {
                    galaxy.0 += scaling_factor;
                }
            }
        }

        let columns = (0..self.width)
            .filter(|j| (0..self.height).all(|i| !self.galaxies.contains(&(i, *j))))
            .rev()
            .collect::<Vec<_>>();

        for j in columns {
            self.width += scaling_factor;
            for galaxy in self.galaxies.iter_mut() {
                if galaxy.1 > j {
                    galaxy.1 += scaling_factor;
                }
            }
        }
    }
}

impl From<&str> for Universe {
    fn from(value: &str) -> Self {
        let galaxies = value
            .lines()
            .enumerate()
            .flat_map(|(i, l)| {
                l.char_indices().filter_map(
                    move |(j, c)| {
                        if c == '#' {
                            Some((i, j))
                        } else {
                            None
                        }
                    },
                )
            })
            .collect();

        Self {
            galaxies,
            width: value.find("\n").expect("couldn't find newline"),
            height: value.lines().count(),
        }
    }
}

impl Display for Universe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.height {
            for j in 0..self.width {
                let c = if self.galaxies.contains(&(i, j)) {
                    "#"
                } else {
                    "."
                };

                write!(f, "{}", c)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
