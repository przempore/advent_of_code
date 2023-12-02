use std::cmp;
use std::collections::HashMap;
use std::fs::read_to_string;

const CONDITION: [(&str, u32); 3] = [("red", 12), ("green", 13), ("blue", 14)];

#[derive(Debug)]
struct Game {
    id: u32,
    subsets: Vec<HashMap<String, u32>>,
    is_possible: bool,
}

fn parse_line(line: &str) -> Game {
    let split = line.split_terminator(':').collect::<Vec<&str>>();
    let id = split[0].split_whitespace().collect::<Vec<&str>>()[1]
        .parse::<u32>()
        .unwrap();

    let subsets = split[1]
        .split_terminator(';')
        .map(|s| s.trim())
        .collect::<Vec<&str>>();
    let subsets = subsets
        .iter()
        .map(|&subset| {
            let sets = subset.split_terminator(',').collect::<Vec<&str>>();

            sets.iter()
                .map(|&s| s.trim())
                .map(|s| {
                    let split = s.split_whitespace().collect::<Vec<&str>>();
                    let number = split[0].parse::<u32>().unwrap();
                    let color = split[1];
                    (color.to_string(), number)
                })
                .collect::<HashMap<String, u32>>()
        })
        .collect::<Vec<_>>();

    Game {
        id,
        subsets,
        is_possible: true,
    }
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| parse_line(line))
        .map(|mut game| {
            game.is_possible = game.subsets.iter().all(|subset| {
                subset.iter().all(|(color, number)| {
                    if let Some((_, n)) = CONDITION.iter().find(|(c, _)| c == color) {
                        return n >= number;
                    }
                    true
                })
            });
            game
        })
        .filter(|game| game.is_possible)
        .map(|game| game.id)
        .sum()
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| parse_line(line))
        .map(|game| {
            let mut max: HashMap<&str, u32> =
                HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);
            for subset in game.subsets.iter() {
                for (color, number) in subset.iter() {
                    if let Some((c, n)) = max.iter().find(|(c, _)| c == &color) {
                        let m = cmp::max(*n, *number);
                        *max.entry(c).or_insert(m) = m;
                    }
                }
            }

            max.iter().map(|(_, &n)| n).product::<u32>()
        })
        .sum()
}

fn main() {
    let input = read_to_string("day2/src/input_part1_example.txt")
        .expect("Error reading input_part1_example.txt");
    let result = part1(&input);
    println!("Example part 1 result: {}", result);

    let input = read_to_string("day2/src/input_part1.txt").expect("Error reading input_part1.txt");
    let result = part1(&input);
    println!("Part 1 result: {}", result);

    let input = read_to_string("day2/src/input_part1_example.txt")
        .expect("Error reading input_part1_example.txt");
    let result = part2(&input);
    println!("Example part 2 result: {}", result);

    let input = read_to_string("day2/src/input_part1.txt").expect("Error reading input_part1.txt");
    let result = part2(&input);
    println!("Part 2 result: {}", result);
}
