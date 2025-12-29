use std::{collections::HashSet, fs::read_to_string};

fn main() {
    let input = read_to_string("input/day7.txt").expect("Should have been able to read file");
    println!("part 1: {}", count_splits(&input));
}

fn count_splits(input: &str) -> usize {
    let start = input
        .lines()
        .next()
        .unwrap()
        .find("S")
        .expect("missing S in first line");

    let mut split_count = 0;
    let mut lasers = HashSet::from([start]);
    for row in input.lines().skip(1) {
        let (count, set) = split_lasers(lasers, row);
        lasers = set;
        split_count += count;
    }

    split_count
}

fn split_lasers(lasers: HashSet<usize>, row: &str) -> (usize, HashSet<usize>) {
    let result = HashSet::new();

    lasers
        .iter()
        .fold((0, result), |(mut split_count, mut set), &val| {
            if let Some(char) = row.get(val..=val) {
                match char {
                    "." => set.insert(val),
                    "^" => {
                        split_count += 1;
                        set.insert(val - 1);
                        set.insert(val + 1)
                    }
                    _ => panic!("invalid char: {char}"),
                };
            }
            (split_count, set)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_lasers() {
        let lasers = HashSet::from([1]);
        let input = ".^.";

        let want_set = HashSet::from([0, 2]);

        let (got_count, got_set) = split_lasers(lasers, input);

        assert_eq!(got_set, want_set);
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
}
