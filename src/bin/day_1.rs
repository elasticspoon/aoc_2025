use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input/day1_1.txt").expect("Should have been able to read file");
    println!("password 1: {}", get_pass(&input));
    println!("password 2: {}", get_pass_complex(&input));
}

fn get_pass(code: &str) -> i32 {
    let mut loc = 50;
    let mut times = 0;
    for line in code.lines() {
        let (_, rem) = rotate(loc, line);
        loc = rem;

        if rem == 0 {
            times += 1;
        }
    }

    times
}

fn get_pass_complex(code: &str) -> i32 {
    let mut loc = 50;
    let mut times = 0;
    for line in code.lines() {
        let (zero_count, place) = rotate(loc, line);
        loc = place;
        times += zero_count;
    }

    times
}

fn rotate(initial: i32, turn: &str) -> (i32, i32) {
    let first_letter = turn.chars().next().expect("Should have direction.");
    let num: i32 = turn[1..].parse().expect("Should have a turn distance.");
    match first_letter {
        'L' => {
            let new_pos = initial - num;
            let mut turns = (new_pos / 100).abs();
            let rem = new_pos.rem_euclid(100);
            if initial > 0 && new_pos <= 0 {
                turns += 1;
            }
            (turns, rem)
        }
        'R' => {
            let new_pos = initial + num;
            let turns = new_pos / 100;
            let rem = new_pos % 100;
            (turns, rem)
        }
        _ => panic!("Expected turned direction of 'L' or 'R'"),
    }
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
        let got = get_pass(moves);
        assert_eq!(got, 3);
    }

    #[test]
    fn test_get_pass_complex() {
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
        let got = get_pass_complex(moves);
        assert_eq!(got, 6);
    }

    #[test]
    fn test_rotate_left() {
        let turn = "L32";
        let (turns, loc) = rotate(50, turn);

        assert_eq!(loc, 18);
        assert_eq!(turns, 0);
    }

    #[test]
    fn test_rotate_right() {
        let turn = "R32";
        let (turns, loc) = rotate(50, turn);

        assert_eq!(loc, 82);
        assert_eq!(turns, 0);
    }

    #[test]
    fn test_rotate_right_exact() {
        let turn = "R50";
        let (turns, loc) = rotate(50, turn);

        assert_eq!(loc, 0);
        assert_eq!(turns, 1);
    }

    #[test]
    fn test_rotate_left_exact_100() {
        let turn = "L101";
        let (turns, loc) = rotate(1, turn);

        assert_eq!(loc, 0);
        assert_eq!(turns, 2);
    }
    #[test]
    fn test_rotate_left_exact_zero() {
        let turn = "L50";
        let (turns, loc) = rotate(50, turn);

        assert_eq!(loc, 0);
        assert_eq!(turns, 1);
    }

    #[test]
    fn test_rotate_left_over() {
        let turn = "L180";
        let (turns, loc) = rotate(50, turn);

        assert_eq!(loc, 70);
        assert_eq!(turns, 2);
    }

    #[test]
    fn test_rotate_left_from_zero() {
        let turn = "L20";
        let (turns, loc) = rotate(0, turn);

        assert_eq!(loc, 80);
        assert_eq!(turns, 0);
    }

    #[test]
    fn test_rotate_right_over() {
        let turn = "R320";
        let (turns, loc) = rotate(50, turn);

        assert_eq!(loc, 70);
        assert_eq!(turns, 3);
    }
}
