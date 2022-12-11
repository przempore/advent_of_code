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

#[derive(Debug)]
struct Path {
    dirs: HashMap<String, u64>,
    path: Vec<String>,
    less_than_100_000: u64,
}

impl Path {
    fn push(&mut self, p: &str) {
        self.path.push(p.to_string());
        // println!(
        //     "Path::push {} = {:?}",
        //     p,
        //     self.get_path_str(self.path.clone())
        // );
    }

    fn pop(&mut self) -> String {
        let path = self.get_path_str(self.path.clone());
        let last_dir_size = self.dirs.get(&path).unwrap().clone();
        let pop = self.path.pop().unwrap_or("".to_string());
        let previous_path_str = self.get_path_str(self.path.clone());
        self.add_to_path(previous_path_str.clone(), last_dir_size);
        if last_dir_size <= 100_000 {
            self.less_than_100_000 += last_dir_size;
        }
        // println!(
        //     "Path::pop | {} | add: {} to path: {} ending with size: {:?}",
        //     line!(),
        //     last_dir_size,
        //     previous_path_str,
        //     self.dirs.get(&previous_path_str)
        // );

        pop
    }

    fn add_to_path(&mut self, path: String, number: u64) {
        self.dirs
            .entry(path)
            .and_modify(|size| *size += number)
            .or_insert(number);
    }

    fn add(&mut self, number: u64) {
        let path_str = self.get_path_str(self.path.clone());
        // println!(
        //     "Path::add | {} | add: {} to path: {}",
        //     line!(),
        //     number,
        //     path_str
        // );
        self.add_to_path(path_str, number);
    }

    fn get_path_str(&mut self, path: Vec<String>) -> String {
        path.iter()
            .map(|p| {
                if p != "/" {
                    return format!("{}/", p);
                }
                p.to_string()
            })
            .collect::<String>()
    }

    fn len(&self) -> usize {
        self.path.len()
    }

    fn get_sum_until(&self, max_size: usize) -> usize {
        let mut dirs_in_vec = self.dirs.iter().collect::<Vec<_>>();
        dirs_in_vec.sort_unstable_by(|a, b| a.1.cmp(b.1));

        let mut in_order_sum = 0u64;
        let mut sums = vec![];
        for (i, (n, s)) in dirs_in_vec.iter().enumerate() {
            if in_order_sum + **s < max_size as u64 {
                in_order_sum += **s;
            }
            let mut sum = **s;
            let part = dirs_in_vec.split_at(i);
            for (nn, ss) in part.1.iter().skip(1) {
                if sum + *ss <= max_size as u64 {
                    sum += *ss;
                    sums.push((format!("{}({}) + {}({}) = {}", n, s, nn, ss, sum), sum));
                }
            }
        }
        println!(
            "sorted by size sums: {:?}",
            sums.iter()
                .map(|(_, s)| *s)
                .collect::<Vec<u64>>()
                .iter()
                .sum::<u64>()
        );
        in_order_sum as usize
    }
}

fn main() -> Result<(), anyhow::Error> {
    let test_input = read_to_string("src/bin/input_day7.txt")?;
    let max_size = 100_000;
    let mut main_path = Path {
        path: vec![],
        dirs: HashMap::new(),
        less_than_100_000: 0,
    };
    for line in test_input.lines() {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        if parts[1] == "ls" {
            continue;
        }
        if parts[1] == "cd" {
            if line.contains("..") {
                let _ = main_path.pop();
            } else {
                let parts = line.split_whitespace().collect::<Vec<&str>>();
                main_path.push(parts[2]);
            }
        } else if line.as_bytes()[0].is_ascii_digit() {
            let parts = line.split_whitespace().collect::<Vec<&str>>();
            main_path.add(parts[0].parse::<u64>().unwrap());
        }
    }

    while main_path.len() > 1 {
        let _ = main_path.pop();
    }

    println!("less_than_100_000: {}", main_path.less_than_100_000);
    println!("total: {:?}", main_path.get_sum_until(max_size));

    Ok(())
}
