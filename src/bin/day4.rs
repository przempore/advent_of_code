use anyhow::Result;
use std::fs::read_to_string;

fn is_entirely_inside(left: (u32, u32), right: (u32, u32)) -> Option<bool> {
    if (left.0 <= right.0 || right.1 >= left.1) && (left.0 >= right.0 || right.1 <= left.1) {
        return Some(true);
    }

    None
}

fn get_pair(line: &str) -> Vec<(u32, u32)> {
    line.split(",")
        .collect::<Vec<_>>()
        .iter()
        .map(|&s| {
            let numbers = s.split("-").collect::<Vec<_>>();

            (
                numbers[0].parse::<u32>().unwrap(),
                numbers[1].parse::<u32>().unwrap(),
            )
        })
        .collect::<Vec<_>>()
}

fn solution_part_one(input_test: &String) -> usize {
    let total = input_test.lines().map(|l| {
        let sides = get_pair(l);
        is_entirely_inside(sides[0], sides[1])
    });

    total.filter(|t| t.is_some()).count()
}

fn is_in_left(left: (u32, u32), n: u32) -> Option<bool> {
    is_entirely_inside(left, (n, n))
}

fn is_in_right(n: u32, right: (u32, u32)) -> Option<bool> {
    is_entirely_inside((n, n), right)
}

fn is_overlaping(left: (u32, u32), right: (u32, u32)) -> Option<bool> {
    if (is_in_left(left, right.0).is_some() || is_in_right(left.1, right).is_some())
        || (is_in_left(right, left.0).is_some() || is_in_right(right.1, left).is_some())
    {
        return Some(true);
    }

    None
}

fn solution_part_two(input_test: &String) -> usize {
    let total = input_test.lines().map(|l| {
        let sides = get_pair(l);
        is_overlaping(sides[0], sides[1])
    });

    total.filter(|t| t.is_some()).count()
}

fn main() -> Result<(), anyhow::Error> {
    let test_input = read_to_string("src/bin/input_day4_test.txt")?;
    println!("test first: {:?}", solution_part_one(&test_input));
    println!("test second: {:?}", solution_part_two(&test_input));
    let input_test = read_to_string("src/bin/input_day4.txt")?;
    println!("solution first: {:?}", solution_part_one(&input_test));
    println!("solution second: {:?}", solution_part_two(&input_test));

    Ok(())
}
