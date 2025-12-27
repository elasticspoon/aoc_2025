use std::{
    collections::HashSet,
    fs::read_to_string,
    ops::{Range, RangeInclusive},
};

fn main() {
    let input = read_to_string("input/day5.txt").expect("Should have been able to read file");
    println!("part 1: {}", count_fresh(&input));
}

fn count_fresh(input: &str) -> u64 {
    let (good_ranges, ids) = input.split_once("\n\n").expect("Should contain '\\n\\n'");
    let good_ranges = fresh_ranges(good_ranges);

    ids.lines()
        .map(|id| {
            id.parse::<u64>()
                .expect("Should be able to parse id to u64")
        })
        .filter(|id| good_ranges.iter().any(|range| range.contains(id)))
        .collect::<Vec<_>>()
        .len() as u64
}

fn fresh_ranges(ids: &str) -> Vec<RangeInclusive<u64>> {
    ids.lines()
        .map(|range| {
            let (start, end) = range.split_once('-').expect("ranges must contain a '-'");
            let (start, end): (u64, u64) = (
                start.parse().expect("cannot convert to u64"),
                end.parse().expect("cannot convert to u64"),
            );
            start..=end
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_fresh_short() {
        let ids = "0-4
6-8

1
2
3
4
5
6
7
8";

        assert_eq!(count_fresh(ids), 7);
    }

    #[test]
    fn test_count_fresh_example() {
        let ids = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

        assert_eq!(count_fresh(ids), 3);
    }
}
