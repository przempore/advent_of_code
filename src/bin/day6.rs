use anyhow::Result;
use std::fs::read_to_string;

fn get_markers_idx(input: &str) -> usize {
    let max_size = 13;
    let mut idx = 1;
    let mut marker = input.chars().nth(0).unwrap().to_string();
    for c in input.chars() {
        while marker.len() > max_size {
            marker.remove(0);
        }
        if marker.contains(c) {
            marker.push(c);
            idx += 1;
        } else {
            marker.push(c);
            idx += 1;
            let mut hash_set = marker.chars().collect::<Vec<_>>();
            hash_set.sort_unstable();
            hash_set.dedup();
            if hash_set.len() == marker.len() {
                return idx - 1;
            }
        }
    }

    idx
}

fn main() -> Result<(), anyhow::Error> {
    let test_input = read_to_string("src/bin/input_day6.txt")?;
    println!("{}", get_markers_idx(&test_input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_test() {
        let test_input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        // assert_eq!(get_markers_idx(test_input), 7);
        assert_eq!(get_markers_idx(test_input), 19);
    }

    #[test]
    fn second_test() {
        let test_input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        // assert_eq!(get_markers_idx(test_input), 5);
        assert_eq!(get_markers_idx(test_input), 23);
    }

    #[test]
    fn third_test() {
        let test_input = "nppdvjthqldpwncqszvftbrmjlhg";
        // assert_eq!(get_markers_idx(test_input), 6);
        assert_eq!(get_markers_idx(test_input), 23);
    }

    #[test]
    fn fourth_test() {
        let test_input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        // assert_eq!(get_markers_idx(test_input), 10);
        assert_eq!(get_markers_idx(test_input), 29);
    }

    #[test]
    fn fifth_test() {
        let test_input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        // assert_eq!(get_markers_idx(test_input), 11);
        assert_eq!(get_markers_idx(test_input), 26);
    }
}
