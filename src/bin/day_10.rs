use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input/day10.txt").expect("Should have been able to read file");
    convert_input(&input);
}

fn convert_input(input: &str) -> Vec<(u16, Vec<u16>, Vec<u16>)> {
    input
        .lines()
        .map(|line| {
            let sb_end = line.find("]").expect("Should contain a ']'");
            let cb_start = line.find("{").expect("Should contain a '{'");

            let light_diagram = line.get(1..sb_end).unwrap();
            let light_diagram: Vec<u8> = light_diagram
                .chars()
                .enumerate()
                .filter_map(|(index, char)| match char {
                    '#' => Some(index as u8),
                    '.' => None,
                    _ => panic!("invalid light fixture: {char}"),
                })
                .collect();
            let light_diagram = indicies_to_bitmask(light_diagram);
            let button_diagrams = line.get((sb_end + 1)..(cb_start - 1)).unwrap().trim();
            let button_diagrams = button_diagrams
                .split(' ')
                .map(|indicies| {
                    let indicies = indicies.trim_start_matches('(').trim_end_matches(')');
                    let indicies = indicies
                        .split(',')
                        .map(|num| {
                            num.parse::<u8>()
                                .unwrap_or_else(|err| panic!("could not covert {num} to u8: {err}"))
                        })
                        .collect();
                    indicies_to_bitmask(indicies)
                })
                .collect::<Vec<u16>>();

            let joltage_indicators = line.get((cb_start + 1)..line.len() - 1).unwrap();
            let joltage_indicators: Vec<u16> = joltage_indicators
                .split(',')
                .map(|num| {
                    num.parse::<u16>()
                        .unwrap_or_else(|err| panic!("could not covert {num} to u16: {err}"))
                })
                .collect();
            (light_diagram, button_diagrams, joltage_indicators)
        })
        .collect()
}

fn indicies_to_bitmask(indicies: Vec<u8>) -> u16 {
    let mut mask: u16 = 0;
    for index in indicies {
        mask |= 1 << index
    }
    mask
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area_points_flat() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        let want = vec![
            (
                0b110,
                vec![0b1000, 0b1010, 0b100, 0b1100, 0b101, 0b11],
                vec![3, 5, 4, 7],
            ),
            (
                0b1000,
                vec![0b11101, 0b1100, 0b10001, 0b111, 0b11110],
                vec![7, 5, 12, 7, 2],
            ),
            (
                0b101110,
                vec![0b11111, 0b11001, 0b110111, 0b110],
                vec![10, 11, 11, 5, 10, 5],
            ),
        ];

        assert_eq!(want, convert_input(input));
    }
}
