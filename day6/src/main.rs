struct RaceRecord {
    duration: usize,
    record_distance: usize,
}

fn main() {
    let example = [RaceRecord {
        duration: 71530,
        record_distance: 940200,
    }];

    let input = [RaceRecord {
        duration: 60808676,
        record_distance: 601116315591300,
    }];

    let answer: usize = input
        .iter()
        .map(|record| {
            (1..record.duration)
                .map(|held_duration| {
                    let remaining_time = record.duration - held_duration;
                    remaining_time * held_duration
                })
                .filter(|d| d > &record.record_distance)
                .count()
        })
        .product();

    dbg!(answer);
}
