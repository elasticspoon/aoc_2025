use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input/day2.txt").expect("Should have been able to read file");
    println!("sum 1: {}", id_sum(&input));
}

fn id_sum(ranges: &str) -> i64 {
    ranges
        .trim_end()
        .split(",")
        .map(|range| {
            let (start, end) = range.split_once("-").unwrap();
            let start: i64 = start
                .parse()
                .unwrap_or_else(|_| panic!("ERROR: could not parse start as i64: {:?}", start));
            let end: i64 = end
                .parse()
                .unwrap_or_else(|_| panic!("ERROR: could not parse end as i64: {:?}", end));

            let mut total = 0;
            for id in start..=end {
                if invalid_id(id) {
                    total += id;
                }
            }

            total
        })
        .sum()
}

fn invalid_id(id: i64) -> bool {
    let digits = id.checked_ilog10().unwrap_or(0) + 1;
    let val = 10_i64.pow(digits / 2);

    id / val == id % val
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id_sum_simple() {
        let id_ranges = "11-22";
        assert_eq!(id_sum(id_ranges), 33)
    }

    #[test]
    fn test_id_sum_multiple() {
        let id_ranges = "11-22,33-44";
        assert_eq!(id_sum(id_ranges), 110)
    }

    #[test]
    fn test_id_sum_example() {
        let id_ranges = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(id_sum(id_ranges), 1227775554)
    }

    #[test]
    fn test_invalid_id_invalid_short() {
        assert!(invalid_id(11))
    }

    #[test]
    fn test_invalid_id_invalid_long() {
        assert!(invalid_id(1234577712345777))
    }

    #[test]
    fn test_invalid_id_valid() {
        assert!(!invalid_id(123456))
    }
}
