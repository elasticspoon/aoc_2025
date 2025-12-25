use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input/day3.txt").expect("Should have been able to read file");
    println!("part 1 total: {}", total_joltage(&input, 2));
    println!("part 2 total: {}", total_joltage(&input, 12));
}

fn total_joltage(banks: &str, digits: usize) -> u64 {
    banks.lines().map(|line| bank_joltage(line, digits)).sum()
}

fn bank_joltage(bank: &str, digits_count: usize) -> u64 {
    let digits: Vec<u64> = bank
        .chars()
        .map(|char| char.to_digit(10).unwrap() as u64)
        .collect();

    let last_index = digits.len() - digits_count;
    bank_joltage_rec(digits, 0, last_index)
}

fn bank_joltage_rec(bank: Vec<u64>, start: usize, end: usize) -> u64 {
    assert!(start <= end);
    if end >= bank.len() {
        return 0;
    }
    let mut max_index = start;
    let mut max_value = bank[max_index];
    for (index, value) in bank[start..=end].iter().enumerate() {
        let index = index + start;
        if *value > max_value {
            max_value = *value;
            max_index = index;
        }
    }
    let pow = bank.len() - end - 1;
    max_value * 10_u64.pow(pow as u32) + bank_joltage_rec(bank, max_index + 1, end + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total_joltage_twelve_digits() {
        let got = total_joltage(
            "987654321111111
811111111111119
234234234234278
818181911112111",
            12,
        );
        assert_eq!(got, 3121910778619);
    }

    #[test]
    fn test_total_joltage_two_digits() {
        let got = total_joltage(
            "987654321111111
811111111111119
234234234234278
818181911112111",
            2,
        );
        assert_eq!(got, 357);
    }

    #[test]
    fn test_bank_jolts_first_two() {
        let digits: Vec<u64> = "987654321111111"
            .chars()
            .map(|char| char.to_digit(10).unwrap() as u64)
            .collect();

        let len = digits.len() - 2;
        let got = bank_joltage_rec(digits, 0, len);
        assert_eq!(got, 98);
    }

    #[test]
    fn test_bank_joltage_first_two() {
        let got = bank_joltage("987654321111111", 2);
        assert_eq!(got, 98);
    }

    #[test]
    fn test_bank_joltage_first_twelve() {
        let got = bank_joltage("987654321111111", 12);
        assert_eq!(got, 987654321111);
    }

    #[test]
    fn test_bank_joltage_first_last() {
        let got = bank_joltage("811111111111119", 2);
        assert_eq!(got, 89);
    }

    #[test]
    fn test_bank_joltage_first_last_twelve() {
        let got = bank_joltage("811111111111119", 12);
        assert_eq!(got, 811111111119);
    }

    #[test]
    fn test_bank_joltage_last_two() {
        let got = bank_joltage("234234234234278", 2);
        assert_eq!(got, 78);
    }

    #[test]
    fn test_bank_joltage_last_twelve() {
        let got = bank_joltage("234234234234278", 12);
        assert_eq!(got, 434234234278);
    }

    #[test]
    fn test_bank_joltage_middle_two() {
        let got = bank_joltage("818181911112111", 2);
        assert_eq!(got, 92);
    }

    #[test]
    fn test_bank_joltage_middle_twelve() {
        let got = bank_joltage("818181911112111", 12);
        assert_eq!(got, 888911112111);
    }

    #[test]
    fn test_bank_joltage_dupes() {
        let got = bank_joltage("99818181911112111", 2);
        assert_eq!(got, 99);
    }

    #[test]
    fn test_bank_joltage_two_dupes_then_biggest() {
        let got = bank_joltage(
            "3432122222342332232122322311242221234222223223223222124222222322212123122323222222222131451231223211",
            2,
        );
        assert_eq!(got, 53);
    }
}
