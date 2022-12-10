use anyhow::Result;
use std::collections::HashMap;
use std::fs::read_to_string;

fn get_path_str(path: &Vec<&str>) -> String {
    path.iter()
        .map(|&p| {
            if p != "/" {
                return format!("{}/", p);
            }
            p.to_string()
        })
        .collect::<String>()
}

fn main() -> Result<(), anyhow::Error> {
    let test_input = read_to_string("src/bin/input_day7.txt")?;
    let max_size = 100000;
    let mut path: Vec<&str> = vec![];
    let mut dirs: HashMap<String, u64> = HashMap::new();
    for line in test_input.lines() {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        if parts[1] == "ls" {
            continue;
        }
        if parts[1] == "cd" {
            if line.contains("..") {
                let path_str = get_path_str(&path);
                let last_dir_size = dirs.get(&get_path_str(&path)).unwrap().clone();
                let _ = path.pop().unwrap();
                let previeus_path_str = get_path_str(&path);
                dirs.entry(previeus_path_str.clone())
                    .and_modify(|size| *size += last_dir_size)
                    .or_insert(0);
                println!(
                    "{} | add: {} to path: {} ending with size: {:?}",
                    line!(),
                    last_dir_size,
                    previeus_path_str,
                    dirs.get(&path_str)
                );
            } else {
                let parts = line.split_whitespace().collect::<Vec<&str>>();
                path.push(parts[2]);
                println!("curent path: {:?}", get_path_str(&path));
                dirs.entry(get_path_str(&path)).or_insert(0);
            }
        } else if line.as_bytes()[0].is_ascii_digit() {
            let path_str = get_path_str(&path);
            println!("{} | add: {} to path: {}", line!(), parts[0], path_str);
            let parts = line.split_whitespace().collect::<Vec<&str>>();
            dirs.entry(path_str)
                .and_modify(|size| *size += parts[0].parse::<u64>().unwrap())
                .or_insert(parts[0].parse::<u64>().unwrap());
        }
    }

    while path.len() > 1 {
        let path_str = get_path_str(&path);
        let _ = path.pop().unwrap();
        let last_dir_size = dirs.get(&path_str).unwrap().clone();
        println!("add: {}:{} to {}", path_str, last_dir_size, get_path_str(&path));
        dirs.entry(get_path_str(&path))
            .and_modify(|size| *size += last_dir_size)
            .or_insert(0);
    }

    let mut dirs_in_vec = dirs.iter().collect::<Vec<_>>();
    dirs_in_vec.sort_unstable_by(|a, b| a.1.cmp(b.1));

    let mut total = 0;
    println!("dirs: {:?}", dirs);
    println!("dirs_in_vec: {:?}", dirs_in_vec);
    for (i, (n, s)) in dirs_in_vec.iter().enumerate() {
        let mut sum = **s;
        let part = dirs_in_vec.split_at(i);
        for (nn, ss) in part.1.iter().skip(1) {
            if sum + *ss < max_size {
                sum += *ss;
                println!("n: {}, nn: {}, s: {}, ss: {}, sum: {}", n, nn, s, ss, sum);
                total = sum;
            }
        }
    }

    println!("total: {:?}", total);

    Ok(())
}
