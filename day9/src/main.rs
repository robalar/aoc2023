use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");

    let answer: i32 = input
        .lines()
        .map(|l| {
            let sequence = l
                .split(" ")
                .map(|x| x.parse::<i32>().expect("could parse integer"))
                .collect::<Vec<_>>();

            get_prev_term(&sequence)
        })
        .sum();

    dbg!(answer);
}

fn get_next_term(sequence: &[i32]) -> i32 {
    let differences = sequence
        .into_iter()
        .tuple_windows()
        .map(|(a, b)| b - a)
        .collect::<Vec<_>>();

    if differences.iter().all(|x| x == &0) {
        sequence[0]
    } else {
        sequence.last().unwrap() + get_next_term(&differences)
    }
}

fn get_prev_term(sequence: &[i32]) -> i32 {
    let differences = sequence
        .into_iter()
        .tuple_windows()
        .map(|(a, b)| b - a)
        .collect::<Vec<_>>();

    if differences.iter().all(|x| x == &0) {
        sequence[0]
    } else {
        sequence.first().unwrap() - get_prev_term(&differences)
    }
}
