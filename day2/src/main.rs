use regex::Regex;
use std::env;
use std::fs;
use std::io;

struct PasswordData {
    min: usize,
    max: usize,
    letter: char,
    password: String,
}

impl PasswordData {
    pub fn new(min: usize, max: usize, letter: char, password: &str) -> Self {
        Self {
            min,
            max,
            letter,
            password: String::from(password),
        }
    }

    pub fn is_valid(&self) -> bool {
        let match_count = self.password.matches(self.letter).count();
        match_count >= self.min && match_count <= self.max
    }

    pub fn is_valid_2(&self) -> bool {
        // why is indexing into a string so complicated in rust
        let pass_chars: Vec<char> = self
            .password
            .as_bytes()
            .iter()
            .map(|&b| b as char)
            .collect();
        (self.letter == pass_chars[self.min - 1] && self.letter != pass_chars[self.max - 1])
            || (self.letter != pass_chars[self.min - 1] && self.letter == pass_chars[self.max - 1])
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
                    letter.as_str().parse::<char>().unwrap(),
                    password.as_str(),
                )),
                _ => None,
            }
        })
        .collect()
}

/// Return the number of valid passwords with methods for part 1 and part 2
fn count_valid(password_datas: &Vec<PasswordData>) -> (u64, u64) {
    let (mut count1, mut count2) = (0, 0);
    for password_data in password_datas {
        if password_data.is_valid() {
            count1 += 1;
        }
        if password_data.is_valid_2() {
            count2 += 1;
        }
    }
    (count1, count2)
}

fn main() {
    let s = input_to_string()
        .expect("Could not read input.txt, make sure it is in the current directory");
    let password_datas = parse_data(&s);
    let (count1, count2) = count_valid(&password_datas);
    println!("Day 1 part 1: {}", count1);
    println!("Day 1 part 2: {}", count2);
}
