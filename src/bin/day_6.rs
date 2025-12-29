use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input/day6.txt").expect("Should have been able to read file");
    println!("part 1: {}", do_math(&input, covert_input_human));
    println!("part 2: {}", do_math(&input, covert_input_ceph));
}

fn do_math(input: &str, coversion_func: fn(input: &str) -> Vec<Vec<String>>) -> u64 {
    let problems = coversion_func(input);

    problems
        .iter()
        .map(|problem| {
            let action = match problem.last().unwrap().as_str() {
                "+" => |lhs: u64, rhs: u64| lhs + rhs,
                "*" => |lhs: u64, rhs: u64| lhs * rhs,
                input => panic!("invalid input: {} in problem: {problem:?}", input),
            };
            let first_val = problem[0]
                .parse::<u64>()
                .unwrap_or_else(|err| panic!("failed to parse {}: {err}", problem[0]));

            problem[1..problem.len() - 1]
                .iter()
                .fold(first_val, |accum, val| {
                    let v = val
                        .parse::<u64>()
                        .unwrap_or_else(|err| panic!("failed to parse {val}: {err}"));
                    action(accum, v)
                })
        })
        .sum()
}

fn covert_input_human(input: &str) -> Vec<Vec<String>> {
    let problem_count = input.lines().next().unwrap().split_whitespace().count();
    let line_count = input.lines().count();
    let mut result: Vec<Vec<String>> = Vec::with_capacity(problem_count);

    for _ in 0..problem_count {
        let problem = Vec::with_capacity(line_count);
        result.push(problem);
    }

    for line in input.lines() {
        for (problem_index, val) in line.split_whitespace().enumerate() {
            result[problem_index].push(val.to_string());
        }
    }
    result
}

fn sizes(inputs: &str) -> Vec<(&str, usize)> {
    let syms: Vec<&str> = inputs.split_whitespace().collect();
    let lens: Vec<usize> = inputs
        .split(['*', '+'])
        .enumerate()
        .map(|(num, spaces)| {
            if num == syms.len() {
                spaces.len() + 1
            } else {
                spaces.len()
            }
        })
        .collect();
    lens[1..lens.len()]
        .iter()
        .zip(syms)
        .map(|(&len, sym)| (sym, len))
        .collect()
}

fn covert_input_ceph(input: &str) -> Vec<Vec<String>> {
    let line_len = input.split_inclusive('\n').next().unwrap().len();
    let line_count = input.lines().count();
    let last_line = input
        .trim_end_matches('\n')
        .split('\n')
        .next_back()
        .unwrap();
    let problem_sizes = sizes(last_line);

    problem_sizes
        .iter()
        .enumerate()
        .map(|(problem_num, (action, problem_size))| {
            let mut strings: Vec<String> = Vec::with_capacity(*problem_size);
            for value_number in (0..*problem_size).rev() {
                let mut buf = String::with_capacity(*problem_size);
                let offset: usize = problem_sizes[0..problem_num]
                    .iter()
                    .map(|(_, len)| *len + 1)
                    .sum();
                for index in 0..(line_count - 1) {
                    let char_index = offset + value_number + index * line_len;
                    let char = input
                        .get(char_index..=char_index)
                        .expect("could not find char at index");
                    if char != " " {
                        buf.push_str(char);
                    }
                }
                strings.push(buf);
            }
            strings.push(action.to_string());

            strings
        })
        .collect::<Vec<Vec<String>>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem_sizes() {
        let input = "* +   *   +  ";
        let want = [("*", 1), ("+", 3), ("*", 3), ("+", 3)];

        assert_eq!(sizes(input), want);
    }

    #[test]
    fn test_covert_input() {
        // 123 328  51 64
        //  45 64  387 23
        //   6 98  215 314
        // *   +   *   +
        let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
";

        let want = [
            ["123", "45", "6", "*"],
            ["328", "64", "98", "+"],
            ["51", "387", "215", "*"],
            ["64", "23", "314", "+"],
        ];

        assert_eq!(covert_input_human(input), want)
    }

    #[test]
    fn test_covert_input_ceph() {
        // 123 328  51 64
        //  45 64  387 23
        //   6 98  215 314
        // *   +   *   +
        let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
";

        let want = [
            ["356", "24", "1", "*"],
            ["8", "248", "369", "+"],
            ["175", "581", "32", "*"],
            ["4", "431", "623", "+"],
        ];

        assert_eq!(covert_input_ceph(input), want)
    }

    #[test]
    fn test_do_math_example_human() {
        // 123 328  51 64
        //  45 64  387 23
        //   6 98  215 314
        // *   +   *   +
        let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
";

        assert_eq!(do_math(input, covert_input_human), 4277556)
    }

    #[test]
    fn test_do_math_example_ceph() {
        // 123 328  51 64
        //  45 64  387 23
        //   6 98  215 314
        // *   +   *   +
        let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
";

        assert_eq!(do_math(input, covert_input_ceph), 3263827)
    }
}
