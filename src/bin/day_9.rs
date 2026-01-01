use std::{collections::HashSet, fs::read_to_string};

fn main() {
    let input = read_to_string("input/day9.txt").expect("Should have been able to read file");
    println!("part 1: {}", largest_rect(&input));
    println!("part 1: {}", largest_rect_in_bounds(&input));
}

fn largest_rect_in_bounds(input: &str) -> usize {
    let points = tuples(input);
    let edge_points = edge_tiles(tuples(input));
    let lower_bound = lower_bound(&edge_points);
    let bounded_points = bounded_points(edge_points, lower_bound);

    let mut max_area = 0;

    for start_point in points.iter() {
        for end_point in points.iter() {
            let area = area(start_point, end_point);
            // println!("testing {start_point:?} -> {end_point:?}");
            if area > max_area && valid_area(start_point, end_point, &bounded_points) {
                // println!("{area} > {max_area}");
                max_area = area;
            }
        }
    }

    max_area
}

fn valid_area(start: &Point, end: &Point, area: &HashSet<Point>) -> bool {
    for x in (start.0.min(end.0))..=start.0.max(end.0) {
        for y in (start.1.min(end.1))..=start.1.max(end.1) {
            if !area.contains(&(x, y)) {
                if start.0 == 2 && start.1 == 3 {
                    // println!("area does not contain {x}, {y}");
                    // println!("{area:?}");
                }
                return false;
            }
        }
    }

    true
}

type Point = (usize, usize);

fn tuples(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|line| {
            let mut nums = line.split(",").map(|num| {
                num.parse::<usize>()
                    .expect("Should be able to parse number")
            });
            (nums.next().unwrap(), nums.next().unwrap())
        })
        .collect()
}

fn area(p1: &Point, p2: &Point) -> usize {
    (p1.0.abs_diff(p2.0) + 1) * (p1.1.abs_diff(p2.1) + 1)
}

fn bounded_points(edges: HashSet<Point>, lower_bound: usize) -> HashSet<Point> {
    let mut points = HashSet::new();

    for &edge_point in edges.iter() {
        let (edge_x, edge_y) = edge_point;
        points.insert((edge_x, edge_y));
        for x_coord in (edge_x + 1)..=lower_bound {
            let target = (x_coord, edge_y);
            if edges.contains(&target) {
                for x_coord in (edge_x + 1)..=x_coord {
                    let target = (x_coord, edge_y);
                    points.insert(target);
                }
                break;
            }
        }
    }

    points
}

fn largest_rect(input: &str) -> usize {
    let points = tuples(input);
    let mut max_area = 0;

    for start_point in points.iter() {
        for end_point in points.iter() {
            let area = area(start_point, end_point);
            if area > max_area {
                max_area = area;
            }
        }
    }

    max_area
}

fn edge_tiles(mut points: Vec<Point>) -> HashSet<Point> {
    points.push(*points.first().unwrap());
    let mut res = HashSet::new();

    for window in points.windows(2) {
        if let (Some(first), Some(second)) = (window.first(), window.get(1)) {
            for x_val in first.0.min(second.0)..=first.0.max(second.0) {
                for y_val in first.1.min(second.1)..=first.1.max(second.1) {
                    res.insert((x_val, y_val));
                }
            }
        }
    }

    res
}

fn lower_bound(points: &HashSet<Point>) -> usize {
    let mut max_x = usize::MIN;
    for &(x, _) in points {
        if x > max_x {
            max_x = x;
        }
    }

    max_x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area_points_flat() {
        let input = HashSet::from([(0, 1), (0, 2), (0, 3), (0, 4)]);
        let want = HashSet::from([(0, 1), (0, 2), (0, 3), (0, 4)]);

        assert_eq!(want, bounded_points(input, 0));
    }

    #[test]
    fn test_area_points_vertical() {
        let input = HashSet::from([(1, 0), (2, 0), (3, 0), (4, 0)]);
        let want = HashSet::from([(1, 0), (2, 0), (3, 0), (4, 0)]);

        assert_eq!(want, bounded_points(input, 4));
    }

    #[test]
    fn test_area_points_square() {
        let input = HashSet::from([
            (0, 0),
            (1, 0),
            (2, 0),
            (2, 1),
            (2, 2),
            (1, 2),
            (0, 2),
            (0, 1),
            (0, 0),
        ]);
        let want = HashSet::from([
            (0, 0),
            (1, 0),
            (2, 0),
            (0, 1),
            (1, 1),
            (2, 1),
            (0, 2),
            (1, 2),
            (2, 2),
        ]);
        assert_eq!(want, bounded_points(input, 4));
    }

    #[test]
    fn test_area_points_m_shape() {
        let input = vec![
            (0, 1),
            (2, 0),
            (2, 1),
            (1, 1),
            (1, 3),
            (2, 3),
            (2, 4),
            (0, 4),
        ];
        let want = HashSet::from([
            (0, 0),
            (1, 0),
            (2, 0),
            (0, 1),
            (1, 1),
            (2, 1),
            (0, 2),
            (1, 2),
            (0, 3),
            (1, 3),
            (2, 3),
            (0, 4),
            (1, 4),
            (2, 4),
        ]);
        assert_eq!(want, bounded_points(edge_tiles(input), 5));
    }

    #[test]
    fn test_edge_tiles_flat() {
        let input = vec![(0, 1), (0, 4)];
        let want = HashSet::from([(0, 1), (0, 2), (0, 3), (0, 4)]);

        assert_eq!(want, edge_tiles(input));
    }

    #[test]
    fn test_edge_tiles_vertical() {
        let input = vec![(1, 0), (4, 0)];
        let want = HashSet::from([(1, 0), (2, 0), (3, 0), (4, 0)]);

        assert_eq!(want, edge_tiles(input));
    }

    #[test]
    fn test_edge_tiles_square() {
        let input = vec![(0, 0), (3, 0), (3, 3), (0, 3)];
        let want = HashSet::from([
            (0, 0),
            (1, 0),
            (2, 0),
            (3, 0),
            (3, 1),
            (3, 2),
            (3, 3),
            (2, 3),
            (1, 3),
            (0, 3),
            (0, 2),
            (0, 1),
            (0, 0),
        ]);
        assert_eq!(want, edge_tiles(input));
    }

    #[test]
    fn test_edge_tiles_example() {
        let input = vec![
            (7, 1),
            (11, 1),
            (11, 7),
            (9, 7),
            (9, 5),
            (2, 5),
            (2, 3),
            (7, 3),
        ];
        let want = HashSet::from([
            (7, 1),
            (8, 1),
            (9, 1),
            (10, 1),
            (11, 1),
            (11, 2),
            (11, 3),
            (11, 4),
            (11, 5),
            (11, 6),
            (11, 7),
            (10, 7),
            (9, 7),
            (9, 6),
            (9, 5),
            (8, 5),
            (7, 5),
            (6, 5),
            (5, 5),
            (4, 5),
            (3, 5),
            (2, 5),
            (2, 4),
            (2, 3),
            (3, 3),
            (4, 3),
            (5, 3),
            (6, 3),
            (7, 3),
            (7, 2),
        ]);
        assert_eq!(want, edge_tiles(input));
    }

    #[test]
    fn test_area_points_example() {
        let input = vec![
            (7, 1),
            (11, 1),
            (11, 7),
            (9, 7),
            (9, 5),
            (2, 5),
            (2, 3),
            (7, 3),
        ];
        let want = HashSet::from([
            (7, 1),
            (8, 1),
            (9, 1),
            (10, 1),
            (11, 1),
            (7, 2),
            (8, 2),
            (9, 2),
            (10, 2),
            (11, 2),
            (7, 3),
            (8, 3),
            (9, 3),
            (10, 3),
            (11, 3),
            (2, 3),
            (3, 3),
            (4, 3),
            (5, 3),
            (6, 3),
            (2, 4),
            (3, 4),
            (4, 4),
            (5, 4),
            (6, 4),
            (7, 4),
            (8, 4),
            (9, 4),
            (10, 4),
            (11, 4),
            (2, 5),
            (3, 5),
            (4, 5),
            (5, 5),
            (6, 5),
            (7, 5),
            (8, 5),
            (9, 5),
            (10, 5),
            (11, 5),
            (9, 6),
            (10, 6),
            (11, 6),
            (9, 7),
            (10, 7),
            (11, 7),
        ]);
        let edge_tiles = edge_tiles(input);
        assert_eq!(want, bounded_points(edge_tiles, 11));
    }

    #[test]
    fn test_largest_rect() {
        let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

        assert_eq!(largest_rect(input), 50);
    }

    #[test]
    fn test_largest_fitting_rect() {
        let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

        assert_eq!(largest_rect_in_bounds(input), 24);
    }
}
