use std::collections::HashMap;

use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");

    let grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let height = grid.len();
    let width = grid[0].len();

    let mut gear_map = HashMap::new();

    for i in 0..height {
        let mut current_number = String::new();
        let mut adjacent_to_gear = None;

        for j in 0..width {
            let c = grid[i][j];
            if c.is_numeric() {
                current_number.push(c);
                if let Some(gear) = has_adjacent_gear(&grid, i, j) {
                    adjacent_to_gear.get_or_insert(gear);
                } 
            } else {
                if let Some(coords) = adjacent_to_gear {
                    gear_map.entry(coords).or_insert(vec![]).push(
                        current_number
                            .parse::<u32>()
                            .expect("could not parse number"),
                    )
                }

                // reset
                current_number = String::new();
                adjacent_to_gear = None;
            }
        }

        if let Some(coords) = adjacent_to_gear {
            gear_map
                .entry(coords)
                .or_insert(vec![])
                .push(current_number.parse().expect("could not parse number"))
        }
    }

    dbg!(&gear_map);

    let answer: u32 = gear_map
        .values()
        .filter_map(|g| {
            if g.len() == 2 {
                Some(g[0] * g[1])
            } else {
                None
            }
        })
        .sum();

    dbg!(answer);
}


fn has_adjacent_gear(grid: &Vec<Vec<char>>, i: usize, j: usize) -> Option<(usize, usize)> {
    for (detla_i, delta_j) in (-1..=1).cartesian_product(-1..=1) {
        let x = j as i32 + delta_j;
        let y = i as i32 + detla_i;

        let adj_c = grid.get(y as usize).and_then(|r| r.get(x as usize));

        if let Some(adj_c) = adj_c {
            if adj_c == &'*' {
                return Some((y as usize, x as usize));
            }
        }
    }

    return None;
}
