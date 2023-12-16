use std::{collections::HashMap, fmt::Display};

fn main() {
    let input = include_bytes!("input.txt");

    let mut boxes = Boxes::new();

    for step in input.split(|b| b == &b',') {
        if let Some(x) = step.iter().position(|b| b == &b'=') {
            let label = &step[0..x];
            let focal_length = std::str::from_utf8(&step[x + 1..])
                .expect("invalid focal length str")
                .parse::<u8>()
                .expect("could not parse focal length");
            let box_i = label.iter().fold(0, hash);
            boxes
                .0
                .entry(box_i)
                .and_modify(|b| {
                    if let Some(l) = b.iter_mut().find(|l| l.label == label) {
                        l.focal_length = focal_length;
                    } else {
                        b.push(Lens {
                            label,
                            focal_length,
                        })
                    }
                })
                .or_insert(vec![Lens {
                    label,
                    focal_length,
                }]);
        } else {
            let label = &step[0..step.len() - 1];
            let box_i = label.iter().fold(0, hash);
            if let Some(lenses) = boxes.0.get_mut(&box_i) {
                if let Some(index) = lenses.iter().position(|l| l.label == label) {
                    lenses.remove(index);
                }
            }
        }
    }

    dbg!(boxes.focusing_power());
}

struct Boxes<'a>(HashMap<u64, Vec<Lens<'a>>>);

impl<'a> Boxes<'a> {
    fn new() -> Self {
        Self(HashMap::new())
    }

    fn focusing_power(&self) -> u64 {
        self.0
            .iter()
            .flat_map(|(i, lenses)| {
                lenses
                    .iter()
                    .enumerate()
                    .map(move |(s, l)| (i + 1) * (s as u64 + 1) * l.focal_length as u64)
            })
            .sum()
    }
}

impl<'a> Display for Boxes<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, lenses) in &self.0 {
            let s = lenses.iter().map(|l| l.to_string()).collect::<String>();
            if !s.is_empty() {
                writeln!(f, "Box {i}: {}", s)?
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
struct Lens<'a> {
    label: &'a [u8],
    focal_length: u8,
}

impl<'a> Display for Lens<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = std::str::from_utf8(self.label).unwrap();
        write!(f, "[{} {}]", label, self.focal_length)
    }
}

fn hash(current: u64, byte: &u8) -> u64 {
    (current + *byte as u64) * 17 % 256
}
