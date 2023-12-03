use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");

    let grid = input.lines().map(|l| l.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

    let height = grid.len();
    let width = grid[0].len();

    let mut sum = 0;
    for i in 0..height {
        let mut current_number = String::new();
        let mut adjacent_to_symbol = false;
        

        for j in 0..width {
            let c = grid[i][j];
            if c.is_numeric() {
                current_number.push(c);
                adjacent_to_symbol |= has_adjacent_symbol(&grid, i, j);
            } else {
                if adjacent_to_symbol && !current_number.is_empty() {
                    sum += current_number.parse::<u32>().expect("could not parse number");
                }

                // reset
                current_number = String::new();
                adjacent_to_symbol = false;
            }
        }

        if adjacent_to_symbol && !current_number.is_empty() {
            sum += current_number.parse::<u32>().expect("could not parse number");
        }
    }

    dbg!(sum);

}


fn has_adjacent_symbol(grid: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    for (detla_i, delta_j) in (-1..=1).cartesian_product(-1..=1) {
        let x = j as i32 + delta_j;
        let y = i as i32 + detla_i;

        let adj_c = grid.get(y as usize).and_then(|r| r.get(x as usize));
        
        if let Some(adj_c) = adj_c {
            if !adj_c.is_numeric() && adj_c != &'.' {
                return true;
            }
        }
    }

    return false;
}