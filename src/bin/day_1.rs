use std::fs::read_to_string;

fn main() {
    part_1();
}

fn part_1() {
    let input = read_to_string("input/day1_1.txt").expect("Should have been able to read file");
    println!("password 1: {}", get_pass(input));
}

fn get_pass(code: String) -> i32 {
    let mut loc = 50;
    let mut times = 0;
    for line in code.lines() {
        // if line.is_empty() {
        //     continue;
        // }
        loc = rotate(loc, line);

        if loc == 0 {
            times += 1;
        }
    }

    times
}

fn rotate(initial: i32, turn: &str) -> i32 {
    let first_letter = turn.chars().next();
    let mut num: i32 = turn[1..].parse().unwrap();
    match first_letter {
        Some('L') => {
            num = initial - num;
        }
        Some('R') => {
            num += initial;
        }
        _ => panic!("unreachable"),
    }

    let mut res = num % 100;
    if res < 0 {
        res += 100;
    };

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_pass_simple() {
        let moves = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        let got = get_pass(moves.to_string());
        assert_eq!(got, 3);
    }

    #[test]
    fn test_rotate_left() {
        let turn = "L32";
        let got = rotate(50, turn);

        assert_eq!(got, 18);
    }

    #[test]
    fn test_rotate_right() {
        let turn = "R32";
        let got = rotate(50, turn);

        assert_eq!(got, 82);
    }

    #[test]
    fn test_rotate_left_over() {
        let turn = "L180";
        let got = rotate(50, turn);

        assert_eq!(got, 70);
    }

    #[test]
    fn test_rotate_right_over() {
        let turn = "R320";
        let got = rotate(50, turn);

        assert_eq!(got, 70);
    }
}
