fn main() {
    let input = include_bytes!("input.txt");
    let answer: u64 = input
        .split(|b| b == &b',')
        .map(|s| s.iter().fold(0u64, |acc, b| (acc + *b as u64) * 17 % 256))
        .sum();
    dbg!(answer);
}
