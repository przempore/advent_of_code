use anyhow::Result;
use std::collections::HashSet;
use std::fs::read_to_string;

const START_LOWER: u8 = b'a';
const START_UPPER: u8 = b'A';

fn get_priority(current_check: &char) -> u32 {
    if current_check.is_ascii_lowercase() {
        (*current_check as u8 - START_LOWER + 1) as u32
    } else {
        (*current_check as u8 - START_UPPER + 27) as u32
    }
}

fn first_part(context: String) -> u32 {
    let context = context
        .lines()
        .map(|l| l.split_at(l.len() / 2))
        .collect::<Vec<_>>();

    let mut sum = 0;
    for (a, b) in context.into_iter() {
        let mut diff: HashSet<u32> = HashSet::new();
        for current_check in a.chars() {
            if b.contains(current_check) {
                diff.insert(get_priority(&current_check));
            }
        }
        sum += diff.iter().sum::<u32>();
    }
    sum
}

fn second_part(context: String) -> u32 {
    let mut threes: Vec<_> = Vec::new();
    let mut lines = context.lines().collect::<Vec<_>>();
    while !lines.is_empty() {
        threes.push(lines.clone().into_iter().rev().take(3).collect::<Vec<_>>());
        let lenght_left = lines.len().saturating_sub(3);
        lines.truncate(lenght_left);
    }

    let priorities: Vec<_> = threes
        .iter()
        .map(|t| {
            if t.len() >= 3 {
                for c in t[0].chars() {
                    if t[1].contains(c) && t[2].contains(c) {
                        return Some(get_priority(&c));
                    }
                }
            }
            None
        })
        .filter_map(|p| p)
        .collect();

    priorities.iter().sum()
}

fn main() -> Result<(), anyhow::Error> {
    let context_test = read_to_string("src/bin/input_day3_test.txt")?;
    println!("test input: {:?}", first_part(context_test.clone()));
    let context = read_to_string("src/bin/input_day3.txt")?;
    println!("first part: {:?}", first_part(context.clone()));

    println!("test input second part: {:?}", second_part(context_test));
    println!("secand part {:?}", second_part(context));
    return Ok(());
}
