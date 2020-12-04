use regex::Regex;
use std::env;
use std::fs;
use std::io;

/// Parses the file in the current directory file into a string
fn file_to_string(filename: &str) -> Result<String, io::Error> {
    let mut file_path = env::current_dir()?;
    file_path.push(&filename);
    fs::read_to_string(file_path)
}

/// Return a vector of passports, each being a string of whitespace-separated key:values
fn read_passports(s: &str) -> Vec<String> {
    s.split("\n\n")
        .map(|passport| passport.replace("\n", " "))
        .collect()
}

fn is_passport_valid(passport: &str) -> bool {
    let fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    for field in fields.iter() {
        let re = Regex::new(&format!(r"{}:.+", field)).unwrap();
        if !re.is_match(passport) {
            return false;
        }
    }
    true
}

fn count_valid_passports(passports: &Vec<String>) -> u64 {
    let mut count = 0;
    for passport in passports {
        if is_passport_valid(passport) {
            count += 1;
        }
    }
    count
}

fn main() {
    let s = file_to_string("input.txt").unwrap();
    let passports = read_passports(&s);
    println!("Part 1: {}", count_valid_passports(&passports));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let s = file_to_string("demo.txt").unwrap();
        let passports = read_passports(&s);
        assert_eq!(count_valid_passports(&passports), 2);
    }
}
