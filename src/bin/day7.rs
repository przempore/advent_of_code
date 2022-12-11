use anyhow::Result;
use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(Debug)]
struct Path {
    dirs: HashMap<String, u64>,
    path: Vec<String>,
    less_than_100_000: u64,
}

impl Path {
    fn push(&mut self, p: &str) {
        self.path.push(p.to_string());
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

    fn get_total(&self, total_space: u64, space_to_delete: u64) -> u64 {
        let free_space = total_space - self.dirs.get("/").unwrap();
        let space_required = space_to_delete - free_space;

        self.dirs
            .iter()
            .filter(|(_, s)| **s >= space_required)
            .map(|(_, s)| return *s)
            .min()
            .unwrap()
    }
}

fn main() -> Result<(), anyhow::Error> {
    let test_input = read_to_string("src/bin/input_day7.txt")?;
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
    println!("part two: get total: {}", main_path.get_total(70000000, 30000000));

    Ok(())
}
