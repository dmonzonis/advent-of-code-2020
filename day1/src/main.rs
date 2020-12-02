use std::env;
use std::fs;
use std::io;

/// Parses the input.txt in the current directory file into a string
fn input_to_string() -> Result<String, io::Error> {
    let mut file_path = env::current_dir()?;
    file_path.push("input.txt");
    fs::read_to_string(file_path)
}

/// Parses a list of numbers from a string containing numbers separated by newlines
fn parse_data(s: &str) -> Vec<i64> {
    s.lines()
        .map(|line| line.parse::<i64>().expect("Error parsing number from data"))
        .collect::<Vec<_>>()
}

/// Find the indices of the first pair of numbers in the list whose sum equals the target
fn find_pair_sum(data: &Vec<i64>, target: i64) -> Option<(usize, usize)> {
    // brute force!
    for i in 0..data.len() {
        for j in (i + 1)..data.len() {
            if data[i] + data[j] == target {
                return Some((i, j));
            }
        }
    }
    None
}

fn find_triplet_sum(data: &Vec<i64>, target: i64) -> Option<(usize, usize, usize)> {
    // brute force!
    for i in 0..data.len() {
        for j in (i + 1)..data.len() {
            for k in (j + 1)..data.len() {
                if data[i] + data[j] + data[k] == target {
                    return Some((i, j, k));
                }
            }
        }
    }
    None
}

fn main() {
    let s = input_to_string().expect(
        "Could not read input file, make sure it is in the current directory and named input.txt",
    );
    let data = parse_data(&s);
    let target = 2020;
    let (i, j) = find_pair_sum(&data, target).expect("No pair of values was found");
    println!("Day 1 part 1: {}", data[i] * data[j]);
    let (i, j, k) = find_triplet_sum(&data, target).expect("No triplet of values was found");
    println!("Day 1 part 2: {}", data[i] * data[j] * data[k]);
}
