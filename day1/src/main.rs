fn main() {
    let input = std::fs::read_to_string("day1/src/input.txt").unwrap();

    let numbers = input
        .lines()
        .filter_map(|line| {
            let number = line
                .chars()
                .filter_map(|char| char.to_digit(10))
                .collect::<Vec<_>>();
            if number.len() > 2 {
                return Some(vec![number[0], number[number.len() - 1]]);
            } else if number.len() == 1 {
                return Some(vec![number[0], number[0]]);
            }
            return Some(number);
        })
        .map(|number| number[0] * 10 + number[1])
        .sum::<u32>();

    println!("{:?}", numbers);
}
