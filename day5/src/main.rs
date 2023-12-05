use std::fs::read_to_string;

struct Almanac {
    seed_to_soil: 
}

fn part1(input: &str) {
    
}

fn main() {
    let input = read_to_string("day5/src/input_part1_example.txt")
        .expect("Error reading day5/src/input_part1_example");

    part1(&input);

    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mapping_parsing() {
        let soil_seed= "50 98 2\n52 50 48";
        let val = soil_seed.lines().fold();
        println!("{}", soil_seed);
    }
}
