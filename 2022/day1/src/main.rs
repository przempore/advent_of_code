fn main() {
    let input = std::fs::read_to_string("2022/day1/src/input_example_part1.txt")
        .expect("2022/day1/src/input_example_part1.txt not found");

    let number = first_part(&input);
    println!("first part example: {:?}", number);

    let input = std::fs::read_to_string("2022/day1/src/input_part1.txt")
        .expect("2022/day1/src/input_part1.txt not found");

    let number = first_part(&input);
    println!("first part: {:?}", number);

    let input = std::fs::read_to_string("2022/day1/src/input_example_part1.txt")
        .expect("2022/day1/src/input_example_part1.txt not found");

    let number = second_part(&input);
    println!("second part example: {:?}", number);

    let input = std::fs::read_to_string("2022/day1/src/input_part1.txt")
        .expect("2022/day1/src/input_part1.txt not found");

    let number = second_part(&input);
    println!("second part: {:?}", number);

}

fn get_sums(input: &str) -> Vec<i32> {
    let mut sums = Vec::new();
    let mut part_sum = 0;

    for l in input.lines() {
        if l.is_empty() {
            sums.push(part_sum);
            part_sum = 0;
            continue;
        }

        part_sum += l.parse::<i32>().unwrap();
    }
    sums.push(part_sum);

    sums
}

fn first_part(input: &str) -> i32 {
    let sums = get_sums(input);

    *sums.iter().max().unwrap()
}

fn second_part(input: &str) -> i32 {
    let mut sums = get_sums(input);
    sums.sort();
    sums.reverse();

    sums[0..3].iter().sum()
}
