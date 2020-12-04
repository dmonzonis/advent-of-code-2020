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

/// Given a key:value string, returns true if value is a number between a and b
fn match_passport_value_range(passport: &str, key: &str, a: u64, b: u64) -> bool {
    let re = Regex::new(&format!(r"{}:(\d{})\b", key, "{4}")).unwrap();
    if let Some(groups) = re.captures(passport) {
        let num = groups.get(1).unwrap().as_str().parse::<u64>();
        match num {
            Ok(num) => {
                if num < a || num > b {
                    return false;
                }
            }
            _ => return false,
        }
    } else {
        return false;
    }
    true
}

fn is_passport_valid_part2(passport: &str) -> bool {
    // first check that it contains all required fields
    if !is_passport_valid(passport) {
        return false;
    }
    // byr
    if !match_passport_value_range(passport, "byr", 1920, 2002) {
        return false;
    }
    // iyr
    if !match_passport_value_range(passport, "iyr", 2010, 2020) {
        return false;
    }
    // eyr
    if !match_passport_value_range(passport, "eyr", 2020, 2030) {
        return false;
    }
    // hgt
    let re = Regex::new(r"hgt:(\d+)(cm|in)").unwrap();
    if let Some(groups) = re.captures(passport) {
        let num = groups.get(1).unwrap().as_str().parse::<u64>();
        if let Err(_) = num {
            return false;
        }
        let num = num.unwrap();
        let unit = groups.get(2).unwrap().as_str();
        let (min, max) = match unit {
            "cm" => (150, 193),
            "in" => (59, 76),
            _ => return false,
        };
        if num < min || num > max {
            return false;
        }
    } else {
        return false;
    }
    // hcl
    let re = Regex::new(r"hcl:#[0-9a-f]{6}\b").unwrap();
    if !re.is_match(passport) {
        return false;
    }
    // ecl
    let re = Regex::new(r"ecl:(amb|blu|brn|gry|grn|hzl|oth)\b").unwrap();
    if !re.is_match(passport) {
        return false;
    }
    // pid
    let re = Regex::new(r"pid:\d{9}\b").unwrap();
    if !re.is_match(passport) {
        return false;
    }
    true
}

fn main() {
    let s = file_to_string("input.txt").unwrap();
    let passports = read_passports(&s);
    // Part 1
    let mut count = 0;
    for passport in &passports {
        if is_passport_valid(&passport) {
            count += 1;
        }
    }
    println!("Part 1: {}", count);
    // Part 2
    count = 0;
    for passport in &passports {
        if is_passport_valid_part2(&passport) {
            count += 1;
        }
    }
    println!("Part 2: {}", count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let s = file_to_string("demo.txt").unwrap();
        let passports = read_passports(&s);
        let mut count = 0;
        for passport in &passports {
            if is_passport_valid(&passport) {
                count += 1;
            }
        }
        assert_eq!(count, 2);
    }

    #[test]
    fn test_part_2_invalid() {
        assert_eq!(
            false,
            is_passport_valid_part2(
                "eyr:1972 cid:100 hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926"
            )
        );
        assert_eq!(
            false,
            is_passport_valid_part2(
                "iyr:2019 hcl:#602927 eyr:1967 hgt:170cm ecl:grn pid:012533040 byr:1946"
            )
        );
        assert_eq!(
            false,
            is_passport_valid_part2(
                "hcl:dab227 iyr:2012 ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277"
            )
        );
        assert_eq!(
            false,
            is_passport_valid_part2(
                "hgt:59cm ecl:zzz eyr:2038 hcl:74454a iyr:2023 pid:3556412378 byr:2007"
            )
        );
    }

    #[test]
    fn test_part_2_valid() {
        assert_eq!(
            true,
            is_passport_valid_part2(
                "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980 hcl:#623a2f"
            )
        );
        assert_eq!(
            true,
            is_passport_valid_part2(
                "eyr:2029 ecl:blu cid:129 byr:1989 iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm"
            )
        );
        assert_eq!(
            true,
            is_passport_valid_part2(
                "hcl:#888785 hgt:164cm byr:2001 iyr:2015 cid:88 pid:545766238 ecl:hzl eyr:2022"
            )
        );
        assert_eq!(
            true,
            is_passport_valid_part2(
                "iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"
            )
        );
    }
}
