use std::fs::read_to_string;
use std::ops::ControlFlow;

fn get_numbers_from_string(line: &str) -> Vec<u32> {
    line.chars()
        .filter_map(|char| char.to_digit(10))
        .collect::<Vec<_>>()
}

fn get_first_and_last_number(number: &Vec<u32>) -> Option<Vec<u32>> {
    Some(vec![*number.first()?, *number.last()?])
}

fn first_part(input: &str) -> u32 {
    input
        .lines()
        .map(|line| get_numbers_from_string(line))
        .filter_map(|number| get_first_and_last_number(&number))
        .map(|number| number[0] * 10 + number[1])
        .sum::<u32>()
}

fn find_word_of_number(word: &str) -> Option<i32> {
    let map = [
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];
    map.iter()
        .find_map(|x| if word.contains(x.0) { Some(x.1) } else { None })
}

fn get_numbers_from_line(line: &str) -> Option<Vec<i32>> {
    let fun = |current: String, x: char| {
        if let Some(dig) = x.to_digit(10)  {
            return ControlFlow::Break(dig as i32);
        }
        if let Some(dig) = find_word_of_number(&current) {
            return ControlFlow::Break(dig);
        }
        return ControlFlow::Continue(current);
    };
    let front = line.chars().enumerate().try_fold(String::new(), |acc, x| {
        let current = format!("{}{}", acc, x.1);
        fun(current, x.1)
    });
    let back = line
        .chars()
        .rev()
        .enumerate()
        .try_fold(String::new(), |acc, x| {
            let current = format!("{}{}", x.1, acc);
            fun(current, x.1)
        });

    if let ControlFlow::Break(front) = front {
        if let ControlFlow::Break(back) = back {
            return Some(vec![front, back]);
        }
    }

    None
}

fn second_part(input: &str) -> i32 {
    input
        .lines()
        .map(|line| get_numbers_from_line(line))
        .filter_map(|number| number)
        .map(|number| number[0] * 10 + number[1])
        .sum::<i32>()
}

fn main() {
    let input = read_to_string("day1/src/input_first_part_example.txt")
        .expect("input_example file not found");
    let numbers = first_part(&input);
    println!("first part example: {:?}", numbers);

    let input = read_to_string("day1/src/input_first_part.txt")
        .expect("input_first_part file not found");
    let numbers = first_part(&input);
    println!("first part: {:?}", numbers);

    let input = read_to_string("day1/src/input_second_part_example.txt")
        .expect("input_second_part_example file not found");
    let numbers = second_part(&input);
    println!("second part example: {:?}", numbers);

    let input = read_to_string("day1/src/input_second_part.txt")
        .expect("input_second_part file not found");
    let numbers = second_part(&input);
    println!("second part: {:?}", numbers);
}
