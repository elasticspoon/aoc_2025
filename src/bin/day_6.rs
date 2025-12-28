use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input/day6.txt").expect("Should have been able to read file");
    println!("part 1: {}", do_math(&input));
}

fn do_math(input: &str) -> u64 {
    let problems = covert_input(input);

    problems
        .iter()
        .map(|problem| {
            let action = match *problem.last().unwrap() {
                "+" => |lhs: u64, rhs: u64| lhs + rhs,
                "*" => |lhs: u64, rhs: u64| lhs * rhs,
                input => panic!("invalid input: {} in problem: {problem:?}", input),
            };
            let first_val = problem[0]
                .parse::<u64>()
                .unwrap_or_else(|err| panic!("failed to parse {}: {err}", problem[0]));

            problem[1..problem.len() - 1]
                .iter()
                .fold(first_val, |accum, &val| {
                    let v = val
                        .parse::<u64>()
                        .unwrap_or_else(|err| panic!("failed to parse {val}: {err}"));
                    action(accum, v)
                })
        })
        .sum()
}

fn covert_input(input: &str) -> Vec<Vec<&str>> {
    let problem_count = input.lines().next().unwrap().split_whitespace().count();
    let line_count = input.lines().count();
    let mut result: Vec<Vec<&str>> = Vec::with_capacity(problem_count);

    for _ in 0..problem_count {
        let problem = Vec::with_capacity(line_count);
        result.push(problem);
    }

    for line in input.lines() {
        for (problem_index, val) in line.split_whitespace().enumerate() {
            result[problem_index].push(val);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_covert_input() {
        let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +";

        let want = [
            ["123", "45", "6", "*"],
            ["328", "64", "98", "+"],
            ["51", "387", "215", "*"],
            ["64", "23", "314", "+"],
        ];

        assert_eq!(covert_input(input), want)
    }

    #[test]
    fn test_do_math_example() {
        let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +";

        assert_eq!(do_math(input), 4277556)
    }
}
