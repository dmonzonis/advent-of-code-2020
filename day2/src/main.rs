use regex::Regex;
use std::env;
use std::fs;
use std::io;

struct PasswordData {
    min: usize,
    max: usize,
    letter: String,
    password: String,
}

impl PasswordData {
    pub fn new(min: usize, max: usize, letter: &str, password: &str) -> Self {
        Self {
            min,
            max,
            letter: String::from(letter),
            password: String::from(password),
        }
    }

    pub fn is_valid(&self) -> bool {
        let match_count = self.password.matches(self.letter.as_str()).count();
        match_count >= self.min && match_count <= self.max
    }
}

/// Parses the input.txt in the current directory file into a string
fn input_to_string() -> Result<String, io::Error> {
    let mut file_path = env::current_dir()?;
    file_path.push("input.txt");
    fs::read_to_string(file_path)
}

fn parse_data(s: &str) -> Vec<PasswordData> {
    let re = Regex::new(r"(\d+)-(\d+)\s([a-z]):\s(\w+)").unwrap();
    re.captures_iter(s)
        .filter_map(|cap| {
            let groups = (cap.get(1), cap.get(2), cap.get(3), cap.get(4));
            match groups {
                (Some(min), Some(max), Some(letter), Some(password)) => Some(PasswordData::new(
                    min.as_str().parse::<usize>().unwrap(),
                    max.as_str().parse::<usize>().unwrap(),
                    letter.as_str(),
                    password.as_str(),
                )),
                _ => None,
            }
        })
        .collect()
}

fn count_valid(password_datas: &Vec<PasswordData>) -> u64 {
    let mut count = 0;
    for password_data in password_datas {
        if password_data.is_valid() {
            count += 1;
        }
    }
    count
}

fn main() {
    let s = input_to_string()
        .expect("Could not read input.txt, make sure it is in the current directory");
    let password_datas = parse_data(&s);
    println!("Day 1 part 1: {}", count_valid(&password_datas));
}
