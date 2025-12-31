use std::cmp::Ordering;
use std::{collections::BinaryHeap, fs::read_to_string};

fn main() {
    let input = read_to_string("input/day8.txt").expect("Should have been able to read file");
    println!("part 1: {}", top_circuits(&input, 1000));
}

#[derive(Eq, Hash, PartialEq, Debug)]
struct Conn {
    dist: usize,
    start: Coord,
    end: Coord,
}

impl Ord for Conn {
    fn cmp(&self, other: &Self) -> Ordering {
        self.dist.cmp(&other.dist)
    }
}

impl PartialOrd for Conn {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Conn {
    fn new(c1: Coord, c2: Coord) -> Self {
        let dist = coord_dist(&c1, &c2);

        if c1 > c2 {
            Self {
                start: c2,
                end: c1,
                dist,
            }
        } else {
            Self {
                start: c1,
                end: c2,
                dist,
            }
        }
    }

    fn is_connecting(&self, other: &Conn) -> bool {
        self.start == other.start
            || self.start == other.end
            || self.end == other.start
            || self.end == other.end
    }
}

const TOP_N_CIRCUITS: usize = 3;
fn top_circuits(input: &str, count: usize) -> usize {
    let tuples = close_tuples(input, count);
    let mut circuits: Vec<Vec<Conn>> = Vec::new();

    for connection in tuples {
        if let Some(circuit) = circuits.iter_mut().find(|circuit| {
            circuit
                .iter()
                .any(|target_conn| target_conn.is_connecting(&connection))
        }) {
            circuit.push(connection);
        } else {
            circuits.push(vec![connection]);
        }
    }

    let mut conn_lens: Vec<usize> = circuits.iter().map(|f| f.len() + 1).collect();
    conn_lens.sort_unstable_by(|a, b| b.cmp(a));
    println!("{conn_lens:?}");
    println!("{}", conn_lens.iter().sum::<usize>());
    conn_lens.truncate(TOP_N_CIRCUITS);

    conn_lens.iter().product()
}

fn close_tuples(input: &str, count: usize) -> Vec<Conn> {
    let tuples = tuples(input);
    let mut res = Vec::new();

    for (index, &current_coord) in tuples.iter().enumerate() {
        for target_coord in tuples.iter().skip(index + 1) {
            res.push(Conn::new(current_coord, *target_coord));
        }
    }

    let mut res = res.into_iter().collect::<Vec<Conn>>();
    res.sort_unstable_by_key(|conn| conn.dist);
    res.truncate(count);

    res
}

fn coord_dist(lhs: &Coord, rhs: &Coord) -> usize {
    lhs.0.abs_diff(rhs.0).pow(2) + lhs.1.abs_diff(rhs.1).pow(2) + lhs.2.abs_diff(rhs.2).pow(2)
}

fn tuples(input: &str) -> Vec<Coord> {
    input
        .lines()
        .map(|line| {
            let mut nums = line.split(",").map(|num| {
                num.parse::<usize>()
                    .expect("Should be able to parse number")
            });
            (
                nums.next().unwrap(),
                nums.next().unwrap(),
                nums.next().unwrap(),
            )
        })
        .collect()
}

type Coord = (usize, usize, usize);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circuits() {
        let input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

        let circuits = top_circuits(input, 10);
        assert_eq!(circuits, 40);
    }

    #[test]
    fn test_closest_tuples_one() {
        let input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

        let got = close_tuples(input, 4);
        let want = Vec::from([
            Conn::new((162, 817, 812), (425, 690, 689)),
            Conn::new((162, 817, 812), (431, 825, 988)),
            Conn::new((906, 360, 560), (805, 96, 715)),
            Conn::new((431, 825, 988), (425, 690, 689)),
        ]);

        assert_eq!(got, want);
    }

    #[test]
    fn test_tuples() {
        let input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

        let got = tuples(input);

        assert_eq!(got.len(), 20);
        assert_eq!(*got.first().unwrap(), (162, 817, 812));
    }
}
