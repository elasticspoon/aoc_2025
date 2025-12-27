use std::{cmp::max, fs::read_to_string, ops::RangeInclusive};

fn main() {
    let input = read_to_string("input/day5.txt").expect("Should have been able to read file");
    println!("part 1: {}", count_fresh(&input));
    println!("part 2: {}", count_possible_fresh(&input));
}

fn count_possible_fresh(input: &str) -> u64 {
    let (input_ranges, _) = input.split_once("\n\n").expect("Should contain '\\n\\n'");
    let good_ranges = fresh_ranges(input_ranges);

    good_ranges
        .iter()
        .map(|range| range.end() - range.start() + 1)
        .sum()
}

fn count_fresh(input: &str) -> u64 {
    let (input_ranges, ids) = input.split_once("\n\n").expect("Should contain '\\n\\n'");
    let good_ranges = fresh_ranges(input_ranges);

    ids.lines()
        .map(|id| {
            id.parse::<u64>()
                .unwrap_or_else(|err| panic!("Should be able to parse id: {id} to u64: {err}"))
        })
        .filter(|id| good_ranges.iter().any(|range| range.contains(id)))
        .count() as u64
}

fn fresh_ranges(ids: &str) -> Vec<RangeInclusive<u64>> {
    let mut vec: Vec<(u64, u64)> = ids
        .lines()
        .map(|range| {
            let (start, end) = range.split_once('-').expect("ranges must contain a '-'");
            (
                start.parse().expect("cannot convert to u64"),
                end.parse().expect("cannot convert to u64"),
            )
        })
        .collect();

    assert!(!vec.is_empty(), "input ranges should not be empty");
    vec.sort_by_key(|(start, _)| *start);

    let mut ranges = Vec::new();

    let (mut current_start, mut current_end) = vec[0];
    for (start, end) in vec {
        if start > current_end {
            ranges.push(current_start..=current_end);
            current_start = start;
            current_end = end;
        } else {
            current_end = max(current_end, end)
        }
    }
    ranges.push(current_start..=current_end);

    ranges
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fresh_ranges_overlapping() {
        let input = "0-2
1-4
6-8";
        let want = Vec::from([0..=4, 6..=8]);

        assert_eq!(fresh_ranges(input), want);
    }

    #[test]
    fn test_fresh_ranges_consuming() {
        let input = "0-9
1-4
6-8";
        let want = Vec::from([0..=9]);

        assert_eq!(fresh_ranges(input), want);
    }

    #[test]
    fn test_fresh_ranges_short() {
        let input = "0-4
6-8";
        let want = Vec::from([0..=4, 6..=8]);

        assert_eq!(fresh_ranges(input), want);
    }

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

    #[test]
    fn test_count_possible_fresh_example() {
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

        assert_eq!(count_possible_fresh(ids), 14);
    }
}
