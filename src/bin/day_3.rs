use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input/day3.txt").expect("Should have been able to read file");
    println!("part 1 total: {}", total_joltage(&input));
}

fn total_joltage(banks: &str) -> u32 {
    banks.lines().map(bank_joltage).sum()
}

fn bank_joltage(bank: &str) -> u32 {
    let digits: Vec<u32> = bank
        .chars()
        .map(|char| char.to_digit(10).unwrap())
        .collect();

    // find the index of the largest number up until the last digit => fisrt_num
    let mut largest_index = 0;
    let mut first_digit = digits[0];
    for index in 0..(digits.len() - 1) {
        if digits[index] > first_digit {
            first_digit = digits[index];
            largest_index = index;
        }
    }

    // find the index of the largest number after first_num
    let mut second_digit = digits[largest_index + 1];
    for index in (largest_index + 1)..(digits.len()) {
        if digits[index] > second_digit {
            second_digit = digits[index];
        }
    }
    let joltage = first_digit * 10 + second_digit;

    println!(
        "initial: {bank} unique string: {}, joltage: {joltage}",
        digits
            .iter()
            .map(|d| d.to_string())
            .collect::<Vec<String>>()
            .join("")
    );

    joltage
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total_joltage() {
        let got = total_joltage(
            "987654321111111
811111111111119
234234234234278
818181911112111",
        );
        assert_eq!(got, 357);
    }
    #[test]
    fn test_bank_joltage_first_two() {
        let got = bank_joltage("987654321111111");
        assert_eq!(got, 98);
    }

    #[test]
    fn test_bank_joltage_first_last() {
        let got = bank_joltage("811111111111119");
        assert_eq!(got, 89);
    }

    #[test]
    fn test_bank_joltage_last_two() {
        let got = bank_joltage("234234234234278");
        assert_eq!(got, 78);
    }

    #[test]
    fn test_bank_joltage_middle_two() {
        let got = bank_joltage("818181911112111");
        assert_eq!(got, 92);
    }

    #[test]
    fn test_bank_joltage_dupes() {
        let got = bank_joltage("99818181911112111");
        assert_eq!(got, 99);
    }

    #[test]
    fn test_bank_joltage_two_dupes_then_biggest() {
        let got = bank_joltage(
            "3432122222342332232122322311242221234222223223223222124222222322212123122323222222222131451231223211",
        );
        assert_eq!(got, 53);
    }
}
