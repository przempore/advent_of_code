use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::read_to_string;

fn get_counted_cards(card: &str) -> HashMap<char, u32> {
    card.chars().fold(HashMap::new(), |mut acc, c| {
        acc.entry(c).and_modify(|c| *c = *c + 1).or_insert(1);
        acc
    })
}

fn get_highest_card(card: &char) -> u32 {
    match card {
        'A' => return 14,
        'K' => return 13,
        'Q' => return 12,
        'J' => return 11,
        'T' => return 10,
        _ => return card.to_digit(10).unwrap(),
    }
}

fn compare_cards(card1: &str, card2: &str) -> Ordering {
    for (c1, c2) in card1.chars().zip(card2.chars()) {
        let c1 = get_highest_card(&c1);
        let c2 = get_highest_card(&c2);
        if c1 > c2 {
            return Ordering::Greater;
        } else if c1 < c2 {
            return Ordering::Less;
        }
    }
    Ordering::Equal
}

fn get_rank(cards: HashMap<char, u32>) -> u32 {
    let max = cards
        .iter()
        .map(|(key, value)| {
            let max = match value {
                1 => 0,
                2 => {
                    for (k, v) in cards.iter() {
                        if *v == 2 && *k != *key {
                            return 2;
                        }
                    }
                    return 1;
                }
                3 => {
                    for (k, v) in cards.iter() {
                        if *v == 2 && *k != *key {
                            return 4;
                        }
                    }
                    return 3;
                }
                4 => 5,
                5 => 6,
                _ => panic!("should never happen"),
            };
            max
        })
        .max()
        .unwrap();
    max
}

fn part1(input: &str) {
    let mut rank_groups = input
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<&str>>())
        .map(|line| (line[0], line[1].parse::<u32>().expect("Error parsing bid")))
        .fold(HashMap::new(), |mut acc, (card, bid)| {
            let rank = get_rank(get_counted_cards(card));
            acc.entry(rank)
                .and_modify(|c: &mut Vec<(&str, u32)>| c.push((card, bid)))
                .or_insert(vec![(card, bid)]);

            acc
        });

    let mut result = 0;
    let mut rank = 1;
    for i in 0..=6 {
        if let Some(cards) = rank_groups.get_mut(&i) {
            cards.sort_by(|(card_a, _), (card_b, _)| compare_cards(card_a, card_b));
            result += cards.iter().fold(0u32, |mut acc, (_, bid)| {
                acc = acc + bid * rank;
                rank += 1;
                acc
            });
        }
    }

    println!("result: {:?}, ", result);
}

fn main() {
    let input = read_to_string("day7/src/input_part1_example.txt")
        .expect("Error reading day7/src/input_part1_example");
    part1(&input);

    let input = read_to_string("day7/src/input_part1.txt")
        .expect("Error reading day7/src/input_part1");
    part1(&input);
}
