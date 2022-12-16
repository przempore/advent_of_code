use anyhow::Result;
use std::fs::read_to_string;

fn first_part(test_input: &str) -> u32 {
    // left -> right
    let mut visible = vec![Vec::<u32>::new()];
    for (l_idx, line) in test_input.lines().enumerate() {
        let mut max = -1;
        for (n_idx, number) in line.chars().enumerate() {
            let number = number.to_digit(10).unwrap() as i32;
            if visible.len() <= l_idx {
                visible.push(vec![]);
            }
            let is_visible = (max < number) as u32;
            if visible[l_idx].len() <= n_idx {
                visible[l_idx].push(is_visible);
            }
            if max < number && visible[l_idx][n_idx] == 0 {
                visible[l_idx][n_idx] = 1;
            }
            if max < number {
                max = number;
            }
        }
    }
    // right -> left
    for (l_idx, line) in test_input.lines().enumerate() {
        let mut max = -1;
        for (n_idx, number) in line.chars().rev().enumerate() {
            let number = number.to_digit(10).unwrap() as i32;
            let len = line.chars().count();
            let n_idx = len - 1 - n_idx;
            if max < number && visible[l_idx][n_idx] == 0 {
                visible[l_idx][n_idx] = 1;
            }
            if max < number {
                max = number;
            }
        }
    }

    // down -> up
    for (l_idx, line) in test_input.lines().enumerate() {
        let mut max = -1i32;
        for (n_idx, _number) in line.chars().enumerate() {
            let len = test_input.lines().count();
            let n_idx = len - 1 - n_idx;
            let number = test_input
                .lines()
                .nth(n_idx)
                .expect("input is not 5 lines long")
                .chars()
                .nth(l_idx)
                .expect("input is not 5 chars long")
                .to_digit(10)
                .expect("invalid number");
            if max < (number as i32) && visible[n_idx][l_idx] == 0 {
                visible[n_idx][l_idx] = 1;
            }
            if max < number as i32 {
                max = number as i32;
            }
        }
    }

    // up -> down
    for (l_idx, line) in test_input.lines().enumerate() {
        let mut max = -1i32;
        for (n_idx, _number) in line.chars().enumerate() {
            let number = test_input
                .lines()
                .nth(n_idx)
                .expect("input is not 5 lines long")
                .chars()
                .nth(l_idx)
                .expect("input is not 5 chars long")
                .to_digit(10)
                .expect("invalid number");
            if max < (number as i32) && visible[n_idx][l_idx] == 0 {
                visible[n_idx][l_idx] = 1;
            }
            if max < number as i32 {
                max = number as i32;
            }
        }
    }

    visible
        .iter()
        .map(|line| line.iter().sum::<u32>())
        .sum::<u32>()
}

fn second_part(test_input: &str) -> u32 {
    unimplemented!();
}

fn main() -> Result<(), anyhow::Error> {
    let test_input = read_to_string("src/bin/input_day8.txt")?;

    println!("first part: {}", first_part(&test_input));
    println!("second part: {}", second_part(&test_input));
    Ok(())
}
