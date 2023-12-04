use std::fs::read_to_string;

fn get_winning_and_owned_numbers(input: &str) -> (Vec<u32>, Vec<u32>) {
    let numbers = input.split_terminator(':').collect::<Vec<&str>>()[1];
    let winning_numbers = numbers.split_terminator('|').collect::<Vec<&str>>()[0];
    let owned_numbers = numbers.split_terminator('|').collect::<Vec<&str>>()[1];

    let parse_numbers = |n: &str| {
        n.split_whitespace()
            .map(|n| n.trim().parse::<u32>())
            .filter_map(|n| n.ok())
            .collect::<Vec<_>>()
    };

    (parse_numbers(winning_numbers), parse_numbers(owned_numbers))
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            let (winning_numbers, owned_numbers) = get_winning_and_owned_numbers(l);

            owned_numbers.iter().fold(0usize, |acc, n| {
                if !winning_numbers.contains(n) {
                    return acc;
                }

                if acc == 0 {
                    acc + 1
                } else {
                    acc * 2
                }
            })
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    let mut cards_points = Vec::new();
    for (idx, l) in input.lines().enumerate() {
        if let Some(_) = cards_points.get(idx) {
            cards_points[idx] += 1;
        } else {
            cards_points.push(1);
        }
        let (winning_numbers, owned_numbers) = get_winning_and_owned_numbers(l);
        let points = owned_numbers
            .iter()
            .filter(|n| winning_numbers.contains(n))
            .count() as u32;

        for copy_idx in idx + 1..idx + 1 + points as usize {
            if let Some(_) = cards_points.get(copy_idx) {
                cards_points[copy_idx] += cards_points[idx];
            } else {
                cards_points.push(0);
                cards_points[copy_idx] += cards_points[idx];
            }
        }
    }

    cards_points.iter().sum()
}

fn main() {
    let input = read_to_string("day4/src/input_part1_example.txt")
        .expect("Error reading input_part1_example.txt");
    let result = part1(&input);
    println!("Example part 1 result: {}", result);

    let input = read_to_string("day4/src/input_part1.txt").expect("Error reading input_part1.txt");
    let result = part1(&input);
    println!("Part 1 result: {}", result);

    let input = read_to_string("day4/src/input_part1_example.txt")
        .expect("Error reading input_part1_example.txt");
    let result = part2(&input);
    println!("Example part 2 result: {}", result);

    let input = read_to_string("day4/src/input_part1.txt").expect("Error reading input_part1.txt");
    let result = part2(&input);
    println!("Part 2 result: {}", result);

}
