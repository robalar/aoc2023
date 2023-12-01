fn main() {
    let input = include_str!("input.txt");

    let answer: usize = input.lines().map(|l| {
        let first_index = l.find(char::is_numeric).expect("could not find numeric char");
        let last_index = l.rfind(char::is_numeric).expect("could not find numeric char");

        let number = format!("{}{}", l.chars().nth(first_index).unwrap(), l.chars().nth(last_index).unwrap()).parse::<usize>().unwrap();
        number
    }).sum();

    dbg!(answer);
}
