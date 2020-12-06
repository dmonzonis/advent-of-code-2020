use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::io;

/// Parses the file in the current directory file into a string
fn file_to_string(filename: &str) -> Result<String, io::Error> {
    let mut file_path = env::current_dir()?;
    file_path.push(&filename);
    fs::read_to_string(file_path)
}

fn parse_group_questions(s: &str) -> Vec<String> {
    s.split("\n\n").map(|line| line.to_owned()).collect()
}

/// Num of answers to which anyone has answered yes
fn count_yes_answers(group: &str) -> u32 {
    let mut answered = HashSet::new();
    for quiz in group.lines() {
        for c in quiz.chars() {
            answered.insert(c);
        }
    }
    answered.len() as u32
}

/// Num of answers to which everyone has answered yes
fn count_yes_answers_all(group: &str) -> u32 {
    let mut answered = HashMap::new();
    let people = group.lines().count() as u32;
    for quiz in group.lines() {
        for c in quiz.chars() {
            let count = answered.entry(c).or_insert(0);
            *count += 1;
        }
    }
    answered.iter().filter(|(_, val)| *val == &people).count() as u32
}

fn main() {
    let s = file_to_string("input.txt").unwrap();
    let groups = parse_group_questions(&s);
    let count = groups.iter().fold(0, |acc, x| acc + count_yes_answers(&x));
    println!("Part 1: {}", count);
    let count = groups
        .iter()
        .fold(0, |acc, x| acc + count_yes_answers_all(&x));
    println!("Part 2: {}", count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let s = file_to_string("demo.txt").unwrap();
        let groups = parse_group_questions(&s);
        assert_eq!(
            11,
            groups.iter().fold(0, |acc, x| acc + count_yes_answers(&x))
        );
    }

    #[test]
    fn test_part_2() {
        let s = file_to_string("demo.txt").unwrap();
        let groups = parse_group_questions(&s);
        assert_eq!(
            6,
            groups
                .iter()
                .fold(0, |acc, x| acc + count_yes_answers_all(&x))
        );
    }
}
