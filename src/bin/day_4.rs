use std::{collections::HashMap, fs::read_to_string, ops::Add};

fn main() {
    let input = read_to_string("input/day4.txt").expect("Should have been able to read file");
    println!("part 1: {}", num_forklift_accessible(&input));
    println!("part 2: {}", num_forklift_accessible_repeat(&input));
}

fn num_forklift_accessible_repeat(grid: &str) -> i32 {
    let mut counts = build_access_counts(grid);
    let mut num_removed = 0;

    loop {
        let (removed_count, updated_counts) = remove_packages(counts);
        counts = updated_counts;
        num_removed += removed_count;

        if removed_count == 0 {
            break;
        }
    }

    num_removed
}

fn num_forklift_accessible(grid: &str) -> i32 {
    let counts = build_access_counts(grid);
    let (removed_count, _) = remove_packages(counts);

    removed_count
}

fn remove_packages(mut counts: HashMap<Coord, i32>) -> (i32, HashMap<Coord, i32>) {
    let values_to_remove: Vec<Coord> = counts
        .iter()
        .filter(|(_, value)| **value < 4)
        .map(|(key, _)| *key)
        .collect();

    let num_removed = values_to_remove.len() as i32;

    for coord in values_to_remove {
        counts.insert(coord, SKIPPABLE);

        for adjacent in &DIRECTIONS {
            let target = &coord + adjacent;
            if let Some(val) = counts.get_mut(&target) {
                *val -= 1;
            }
        }
    }

    (num_removed, counts)
}

fn build_access_counts(grid: &str) -> HashMap<Coord, i32> {
    let lines: Vec<&str> = grid.lines().collect();
    let y_len = lines.len();
    let x_len = lines.first().unwrap().len();
    let mut access: HashMap<Coord, i32> = HashMap::with_capacity(x_len * y_len);

    for (y_index, line) in lines.iter().enumerate() {
        for (x_index, char) in line.chars().enumerate() {
            match char {
                '.' => {
                    let current = Coord(x_index as i32, y_index as i32);
                    access.insert(current, SKIPPABLE);
                }
                '@' => {
                    let current = Coord(x_index as i32, y_index as i32);
                    access.entry(current).or_insert(0);

                    let current = Coord(x_index as i32, y_index as i32);
                    for adjacent in &DIRECTIONS {
                        let target = &current + adjacent;
                        if target.valid(x_len as i32, y_len as i32) {
                            *access.entry(target).or_insert(0) += 1;
                        }
                    }
                }
                _ => panic!("char must be '@' or '.'. got: {}", char),
            }
        }
    }

    access
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct Coord(i32, i32);

impl Coord {
    fn valid(&self, x_max: i32, y_max: i32) -> bool {
        self.0 >= 0 && self.0 < x_max && self.1 >= 0 && self.1 < y_max
    }
}

const SKIPPABLE: i32 = 99;
const DIRECTIONS: [Coord; 8] = [
    Coord(-1, -1),
    Coord(-1, 0),
    Coord(-1, 1),
    Coord(0, -1),
    Coord(0, 1),
    Coord(1, -1),
    Coord(1, 0),
    Coord(1, 1),
];

impl Add for &Coord {
    type Output = Coord;

    fn add(self, rhs: Self) -> Self::Output {
        Coord(self.0 + rhs.0, self.1 + rhs.1)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_forklift_access() {
        let input = "@@@
@@@
@@@";
        let want = HashMap::from([
            (Coord(0, 0), 3),
            (Coord(0, 1), 5),
            (Coord(0, 2), 3),
            (Coord(1, 0), 5),
            (Coord(1, 1), 8),
            (Coord(1, 2), 5),
            (Coord(2, 0), 3),
            (Coord(2, 1), 5),
            (Coord(2, 2), 3),
        ]);
        let got = build_access_counts(input);
        assert_eq!(got, want);
    }

    #[test]
    fn test_num_forklift_accessible() {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        let got = num_forklift_accessible(input);
        assert_eq!(got, 13);
    }

    #[test]
    fn test_num_forklift_accessible_repeat() {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        let got = num_forklift_accessible_repeat(input);
        assert_eq!(got, 43);
    }
}
