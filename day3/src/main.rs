use std::fs::read_to_string;

fn part1(input: &str) -> u32 {
    let mut numbers = Vec::new();
    let mut signs = Vec::new();
    let mut current_number = String::new();
    let mut coord = (0, 0);
    for (line_idx, line) in input.lines().enumerate() {
        for (i, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                if current_number.is_empty() {
                    coord.0 = i;
                    coord.1 = i;
                } else {
                    coord.1 = i;
                }
                current_number.push(c);
            } else {
                if !current_number.is_empty() {
                    let parsed = current_number.parse::<u32>().unwrap();
                    println!("num: {}, parsed: {}", current_number, parsed);
                    numbers.push((parsed, (line_idx, coord)));
                    current_number.clear();
                }
                if c != '.' {
                    signs.push((c, (line_idx, i)));
                }
            }
        }
    }

    // println!("numbers: {:?}", numbers);
    // println!("signs: {:?}", signs);

    numbers
        .iter()
        .map(|(number, (line_idx, (begin, end)))| {
            signs.iter().find_map(|(_, (l_idx, column_idx))| {
                if line_idx == l_idx {
                    if column_idx + 1 == *begin || *column_idx == end + 1 {
                        // same line
                        print!("{} ", number);
                        return Some(*number);
                    }
                    return None;
                } else if *line_idx == l_idx + 1 || line_idx + 1 == *l_idx {
                    // sign is under the number
                    if column_idx + 1 >= *begin && *column_idx <= end + 1 {
                        // same line
                        print!("{} ", number);
                        return Some(*number);
                    }
                    return None;
                }
                None
            })
        })
        .filter_map(|number| number)
        .sum()
}

fn main() {
    let input = read_to_string("day3/src/input_part1_example.txt")
        .expect("Error reading input_part1_example.txt");
    let result = part1(&input);
    println!("Example part 1 result: {}", result);

    let input = read_to_string("day3/src/input_part1.txt").expect("Error reading input_part1.txt");
    let result = part1(&input);
    println!("Part 1 result: {}", result);
    //
    // let input = read_to_string("day3/src/input_part1_example.txt")
    //     .expect("Error reading input_part1_example.txt");
    // let result = part2(&input);
    // println!("Example part 2 result: {}", result);
    //
    // let input = read_to_string("day3/src/input_part1.txt").expect("Error reading input_part1.txt");
    // let result = part2(&input);
    // println!("Part 2 result: {}", result);
}
