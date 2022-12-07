use anyhow::Result;
use std::fs::read_to_string;
use std::iter::Iterator;

#[derive(Debug, Default, Clone)]
struct Column {
    body: String,
}

fn parse_column(line: &str) -> Vec<Option<char>> {
    let iter = line.chars().skip(1);

    let mut line = vec![];
    for c in iter.step_by(4) {
        if c.is_ascii() {
            line.push(Some(c));
        } else {
            line.push(None);
        }
    }

    line
}

fn read_columns(test_input: &str) -> Vec<Column> {
    let columns_chars = test_input
        .lines()
        .filter(|l| l.contains("["))
        .map(|l| parse_column(l))
        .collect::<Vec<_>>();

    let mut columns = vec![];
    for (col_idx, row) in columns_chars.iter().enumerate() {
        for (idx, piece) in row.iter().enumerate() {
            if col_idx == 0 {
                if let Some(piece) = piece {
                    columns.push(Column {
                        body: piece.to_string(),
                    });
                }
                continue;
            }
            if let Some(piece) = piece {
                columns[idx].body.push(*piece);
            }
        }
    }

    let columns = columns
        .iter()
        .map(|c| Column {
            body: c.body.trim().to_string(),
        })
        .collect::<Vec<_>>();

    columns
}

#[derive(Debug, Default)]
struct Command {
    amount: usize,
    from: usize,
    to: usize,
}

fn parse_commands(line: &str) -> Command {
    let split: Vec<&str> = line.split_whitespace().collect();
    Command {
        amount: split[1].parse::<usize>().unwrap(),
        from: split[3].parse::<usize>().unwrap() - 1,
        to: split[5].parse::<usize>().unwrap() - 1,
    }
}

fn read_commands(test_input: &str) -> Vec<Command> {
    test_input
        .lines()
        .filter(|l| l.contains("move"))
        .map(|l| parse_commands(l))
        .collect::<Vec<_>>()
}

fn first_part(commands: &Vec<Command>, mut columns: Vec<Column>) -> String {
    for com in commands.iter() {
        for _ in 0..com.amount {
            let what = columns[com.from].body.remove(0); // that remove is pretty costly, probably
                                                         // `pop` and `push` in reversed `String`
                                                         // be more performent?
            columns[com.to].body = format!("{}{}", what, columns[com.to].body);
        }
    }

    columns
        .iter()
        .map(|c| c.body.chars().next().unwrap())
        .collect::<String>()
}

fn second_part(commands: &Vec<Command>, mut columns: Vec<Column>) -> String {
    for com in commands.iter() {
        let what = columns[com.from].body.clone();
        let what = what.get(0..com.amount).unwrap();
        for _ in 0..com.amount {
            columns[com.from].body.remove(0);
        }
        columns[com.to].body = format!("{}{}", what, columns[com.to].body);
    }

    columns
        .iter()
        .map(|c| c.body.chars().next().unwrap())
        .collect::<String>()
}

fn main() -> Result<(), anyhow::Error> {
    let test_input = read_to_string("src/bin/input_day5.txt")?;
    let columns = read_columns(&test_input);
    let commands = read_commands(&test_input);

    let result = first_part(&commands, columns.clone());
    println!("{}", result);

    let result = second_part(&commands, columns);
    println!("{}", result);

    Ok(())
}
