use std::fs::read_to_string;

fn calculate_number_of_winning_options(time: u64, record_distance: u64) -> u64 {
    let mut winning_options_count = 0;

    for hold_time in 1..time {
        let speed = hold_time;
        let travel_time = time - hold_time;
        let distance = speed * travel_time;

        if distance > record_distance {
            winning_options_count += 1;
        }
    }

    winning_options_count
}

fn part1(input: &str) {
    let times = input
        .lines()
        .map(|line| {
            let times = line.split_whitespace().collect::<Vec<&str>>();
            times
                .iter()
                .map(|&line| line.trim())
                .filter(|line| !line.contains(":"))
                .map(|time| time.parse::<u64>().expect("Error parsing time to u64"))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let winning_options = times[0]
        .iter()
        .zip(times[1].iter())
        .fold(1u64, |acc, (time, distance)| {
            acc * calculate_number_of_winning_options(*time, *distance)
        });
    println!("Winning options: {:?}", winning_options);
}

fn part2(input: &str) {
    let times = input
        .lines()
        .map(|line| {
            let line = line.replace("Time:", "");
            let line = line.replace("Distance:", "");
            let times = line.split_whitespace().collect::<String>();
            times.parse::<u64>().expect("Error parsing time to u64")
        })
        .collect::<Vec<_>>();

    let winning_options = calculate_number_of_winning_options(times[0], times[1]);

    println!("Winning options: {:?}", winning_options);
}

fn main() {
    let input = read_to_string("day6/src/input_part1_example.txt")
        .expect("Error reading day6/src/input_part1_example");

    part1(&input);
    let input = read_to_string("day6/src/input_part1.txt")
        .expect("Error reading day6/src/input_part1");
    part1(&input);

    let input = read_to_string("day6/src/input_part1_example.txt")
        .expect("Error reading day6/src/input_part1_example");
    part2(&input);

    let input =
        read_to_string("day6/src/input_part2.txt").expect("Error reading day6/src/input_part2");
    part2(&input);
}
