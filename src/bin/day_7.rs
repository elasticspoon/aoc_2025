use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let input = read_to_string("input/day7.txt").expect("Should have been able to read file");
    println!("part 1: {}", count_splits(&input));
    println!("part 2: {}", count_timelines(&input));
}

fn count_timelines(input: &str) -> usize {
    let start = input
        .lines()
        .next()
        .unwrap()
        .find("S")
        .expect("missing S in first line");

    let mut lasers = HashMap::from([(start, 1)]);
    for row in input.lines().skip(1) {
        let (_, set) = split_lasers(lasers, row);
        lasers = set;
    }

    lasers.values().sum()
}

fn count_splits(input: &str) -> usize {
    let start = input
        .lines()
        .next()
        .unwrap()
        .find("S")
        .expect("missing S in first line");

    let mut split_count = 0;
    let mut lasers = HashMap::from([(start, 1)]);
    for row in input.lines().skip(1) {
        let (count, set) = split_lasers(lasers, row);
        lasers = set;
        split_count += count;
    }

    split_count
}

fn split_lasers(lasers: HashMap<usize, usize>, row: &str) -> (usize, HashMap<usize, usize>) {
    let result = HashMap::new();

    lasers.into_iter().fold(
        (0, result),
        |(mut split_count, mut set), (val, timeline_count)| {
            if let Some(char) = row.get(val..=val) {
                match char {
                    "." => {
                        set.entry(val)
                            .and_modify(|v| *v += timeline_count)
                            .or_insert(timeline_count);
                    }
                    "^" => {
                        split_count += 1;
                        set.entry(val - 1)
                            .and_modify(|v| *v += timeline_count)
                            .or_insert(timeline_count);
                        set.entry(val + 1)
                            .and_modify(|v| *v += timeline_count)
                            .or_insert(timeline_count);
                    }
                    _ => panic!("invalid char: {char}"),
                };
            }
            (split_count, set)
        },
    )
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_split_lasers_basic() {
        let lasers = HashMap::from([(1, 1)]);
        let input = ".^.";

        let want_map = HashMap::from([(0, 1), (2, 1)]);

        let (got_count, got_set) = split_lasers(lasers, input);

        assert_eq!(got_set, want_map);
        assert_eq!(got_count, 1);
    }

    #[test]
    fn test_split_lasers_overlaps() {
        let lasers = HashMap::from([(0, 1), (1, 2)]);
        let input = ".^.";

        let want_map = HashMap::from([(0, 3), (2, 2)]);

        let (got_count, got_set) = split_lasers(lasers, input);

        assert_eq!(got_set, want_map);
        assert_eq!(got_count, 1);
    }

    #[test]
    fn test_count_splits() {
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

        assert_eq!(count_splits(input), 21);
    }

    #[test]
    fn test_count_timelines() {
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

        assert_eq!(count_timelines(input), 40);
    }
}
