use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let input = read_to_string("input/day7.txt").expect("Should have been able to read file");
    println!("part 1: {}", count_splits(&input));
    println!("part 2: {}", count_timelines(&input));
}

fn count_timelines(input: &str) -> usize {
    let start = start_index(input);
    let lasers = HashMap::from([(start, 1)]);
    input
        .lines()
        .skip(1)
        .fold(lasers, |map, row| split_lasers(map, row).1)
        .values()
        .sum()
}

fn start_index(input: &str) -> usize {
    input
        .lines()
        .next()
        .and_then(|first_line| first_line.find('S'))
        .expect("missing S in first line")
}

fn count_splits(input: &str) -> usize {
    let start = start_index(input);
    let lasers = HashMap::from([(start, 1)]);
    input
        .lines()
        .skip(1)
        .fold((0, lasers), |(split_count, map), row| {
            let (count, new_map) = split_lasers(map, row);
            (split_count + count, new_map)
        })
        .0
}

fn split_lasers(lasers: HashMap<usize, usize>, row: &str) -> (usize, HashMap<usize, usize>) {
    lasers.into_iter().fold(
        (0, HashMap::new()),
        |(mut split_count, mut laser_pos_set), (laser_pos, timeline_count)| {
            if let Some(char) = row.as_bytes().get(laser_pos) {
                match char {
                    b'.' => {
                        *laser_pos_set.entry(laser_pos).or_insert(0) += timeline_count;
                    }
                    b'^' => {
                        split_count += 1;
                        *laser_pos_set.entry(laser_pos - 1).or_insert(0) += timeline_count;
                        *laser_pos_set.entry(laser_pos + 1).or_insert(0) += timeline_count;
                    }
                    _ => panic!("invalid char: {char}"),
                };
            }
            (split_count, laser_pos_set)
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
