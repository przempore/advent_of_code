use std::fs::read_to_string;

#[derive(Debug, Default)]
struct Product {
    source: String,
    destination: String,
    source_beginnings: Vec<u64>,
    target_begginings: Vec<u64>,
    lengths: Vec<u64>,
}

impl Product {
    fn parse_config(&mut self, config: &str) {
        let target_source = config.split_whitespace().collect::<Vec<_>>()[0];
        let target_source = target_source.split_terminator('-').collect::<Vec<_>>();
        self.source = target_source[0].to_string();
        self.destination = target_source[2].to_string();
    }

    fn parse_numbers(&mut self, numbers: &str) {
        let numbers = numbers
            .split_whitespace()
            .map(|n| n.parse::<u64>().expect("Error parsing products numbers"))
            .collect::<Vec<_>>();
        self.target_begginings.push(numbers[0]);
        self.source_beginnings.push(numbers[1]);
        self.lengths.push(numbers[2]);
    }

    fn get_destination(&self, source: u64) -> u64 {
        let destination = self
            .source_beginnings
            .iter()
            .zip(self.target_begginings.iter())
            .zip(self.lengths.iter())
            .find_map(|((&s, &t), &l)| {
                if source >= s && source < s + l {
                    return Some(t + (source - s));
                }
                None
            });

        if let Some(d) = destination {
            d
        } else {
            source
        }
    }
}

fn part1(input: &str) -> Option<u64> {
    let seeds = input.lines().take(1).map(|l| l).collect::<String>();
    let numbers = seeds.split_terminator(':').collect::<Vec<_>>();
    let seeds = numbers[1]
        .trim()
        .split_whitespace()
        .map(|n| n.parse::<u64>().expect("Error parsing seeds numbers"))
        .collect::<Vec<_>>();

    let products =
        input
            .lines()
            .skip(1)
            .filter(|l| !l.is_empty())
            .fold(Vec::new(), |mut acc, l| {
                let mut product = Product::default();
                if l.contains("map") {
                    product.parse_config(l);
                    acc.push(product);
                } else {
                    if let Some(last) = acc.last_mut() {
                        last.parse_numbers(l);
                    }
                }
                acc
            });

    let locations = seeds
        .iter()
        .map(|s| {
            let destination = products.iter().fold(*s, |seed, product| {
                let destination = product.get_destination(seed);
                println!(
                    "{}[{}] -> {}[{}]",
                    product.source, seed, product.destination, destination
                );
                destination
            });
            destination
        })
        .collect::<Vec<_>>();

    locations.iter().min().copied()
}

fn main() {
    let input = read_to_string("day5/src/input_part1_example.txt")
        .expect("Error reading day5/src/input_part1_example");
    let result = part1(&input);
    if let Some(result) = result {
        println!("Part 1 example result: {:?}", result);
    }

    let input =
        read_to_string("day5/src/input_part1.txt").expect("Error reading day5/src/input_part1");
    let result = part1(&input);
    if let Some(result) = result {
        println!("Part 1 result: {:?}", result);
    }
}
