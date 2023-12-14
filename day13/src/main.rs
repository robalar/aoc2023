fn main() {
    let input = include_str!("input.txt");
    let answer: usize = input.split("\n\n").map(find_mirror_line).sum();

    dbg!(answer);
}

fn find_mirror_line(pattern: &str) -> usize {
    let rows = pattern.lines().collect::<Vec<_>>();

    let height = rows.len();

    // Scan for horizontal reflections with one difference
    for i in 1..height {
        let above = &rows[0..i];
        let below = &rows[i..height];

        let zipped = above.iter().rev().zip(below);
        if zipped
            .map(|(a, b)| {
                a.chars()
                    .zip(b.chars())
                    .filter(|(a_c, b_c)| a_c != b_c)
                    .count()
            })
            .sum::<usize>()
            == 1
        {
            return 100 * i;
        }
    }

    // Scan for vertical reflections with one difference
    let columns: Vec<String> = (0..rows[0].len())
        .map(|i| {
            rows.iter()
                .map(|c| c.chars().nth(i).unwrap())
                .collect::<String>()
        })
        .collect();

    let width = columns.len();

    for i in 1..width {
        let left = &columns[0..i];
        let right = &columns[i..width];

        let zipped = left.iter().rev().zip(right);
        if zipped
            .map(|(a, b)| {
                a.chars()
                    .zip(b.chars())
                    .filter(|(a_c, b_c)| a_c != b_c)
                    .count()
            })
            .sum::<usize>()
            == 1
        {
            return i;
        }
    }

    panic!("No reflection!:\n{}", pattern);
}
