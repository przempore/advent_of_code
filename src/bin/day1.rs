fn main() {
    let file = include_str!("./input_day1.txt");

    let mut total = 0;
    let mut elves_calories = Vec::<u32>::new();
    let _: Vec<u32> = file
        .split("\n")
        .filter_map(|x| {
            let x = match x.parse::<u32>() {
                Ok(x) => {
                    total += x;
                    Some(x)
                }
                Err(_) => {
                    elves_calories.push(total);
                    total = 0;
                    None
                }
            };
            x
        })
        .collect();

    elves_calories.sort_by(|a, b| {
        let x = b.cmp(a);
        x
    });
    println!("First part: {}", elves_calories.first().unwrap());

    if elves_calories.len() > 3 {
        println!("Second part: {}", elves_calories[0..3].iter().sum::<u32>());
    }
}
