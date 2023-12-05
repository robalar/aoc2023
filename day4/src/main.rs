use std::collections::{HashSet, HashMap};

fn main() {
    let input = include_str!("input.txt");
    
    let card_map: HashMap<_, _> = input.lines().map(|l| {
        let (header, card) = l.split_once(":").expect("Could not split at ':'");

        let id: u32 = header.strip_prefix("Card").and_then(|s| s.trim().parse().ok()).expect("could not parse id");

        let (winning, have) = card.split_once("|").expect("could not split at '|'");

        let winning = winning.split(" ").filter_map(|s| s.parse::<u32>().ok()).collect::<HashSet<_>>();
        let have = have.split(" ").filter_map(|s| s.parse::<u32>().ok()).collect::<HashSet<_>>();

        let matches = winning.intersection(&have);

        (id, matches.count() as u32)
    }).collect();

    let mut total = 0;
    let mut stack: Vec<_> = card_map.iter().collect();

    while let Some((id, count)) = stack.pop() {
        total += 1;
        for i in id+1..=id+count {
            stack.push(card_map.get_key_value(&i).expect("could not get card from map"))
        }
    }

    dbg!(total);
}
