fn first_part(input: &str) -> u32 {
    input
        .lines()
        .filter_map(|line| {
            let number = line
                .chars()
                .filter_map(|char| char.to_digit(10))
                .collect::<Vec<_>>();
            if number.len() > 2 {
                return Some(vec![number[0], number[number.len() - 1]]);
            } else if number.len() == 1 {
                return Some(vec![number[0], number[0]]);
            }
            return Some(number);
        })
        .map(|number| number[0] * 10 + number[1])
        .sum::<u32>()
}

fn second_part(input: &str) -> u32 {
    panic!("not implemented")
}


fn main() {
    let input = std::fs::read_to_string("day1/src/input_first_part_example.txt")
        .expect("input_example file not found");
    let numbers = first_part(&input);
    println!("first part example: {:?}", numbers);

    let input = std::fs::read_to_string("day1/src/input_first_part.txt")
        .expect("input_first_part file not found");
    let numbers = first_part(&input);
    println!("first part: {:?}", numbers);

    let input = std::fs::read_to_string("day1/src/input_second_part_example.txt")
        .expect("input_second_part_example file not found");
    let numbers = second_part(&input);
    println!("second part example: {:?}", numbers);
}
