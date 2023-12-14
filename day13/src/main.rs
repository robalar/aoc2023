fn main() {
    let input = include_str!("input.txt");
    let answer: usize = input.split("\n\n").map(find_mirror_line).sum();

    dbg!(answer);
}

fn find_mirror_line(pattern: &str) -> usize {
    let rows = pattern.lines().collect::<Vec<_>>();

    let height = rows.len();

    // Scan for horizontal reflections
    for i in 1..height {
        let above = &rows[0..i];
        let below = &rows[i..height];

        let mut zipped = above.iter().rev().zip(below);
        if zipped.all(|(a, b)| a == b) {
            return 100 * i;
        }
    }

    // Scan for vertical reflections
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

        let mut zipped = left.iter().rev().zip(right);
        if zipped.all(|(a, b)| a == b) {
            return i;
        }
    }

    panic!("No reflection!:\n{}", pattern);
}
